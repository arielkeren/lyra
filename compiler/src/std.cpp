#include "std.hpp"

#include <cmath>
#include <cstring>
#include <sstream>

std::unordered_map<std::string, std::function<Value(const std::vector<Value>&)>>
    Value::global_methods_;

// Constructors
Value::Value(const Value& other)
    : type_(other.type_), value_(other.value_), list_(other.list_) {}

Value::Value(std::nullptr_t) : type_(Type::Null) {}

Value::Value(int value)
    : type_(Type::Int), value_(static_cast<double>(value)) {}

Value::Value(double value) : type_(Type::Float), value_(value) {}

Value::Value(bool value) : type_(Type::Bool), value_(value ? 1.0 : 0.0) {}

Value::Value(char value)
    : type_(Type::Char), value_(static_cast<double>(value)) {}

Value::Value(const char* value) : type_(Type::String) {
    list_.reserve(strlen(value));
    for (const char* p = value; *p != '\0'; ++p) list_.push_back(Value(*p));
}

Value::Value(const std::vector<Value>& value)
    : type_(Type::List), list_(value) {}

// Assignment operators
Value& Value::operator=(const Value& other) {
    if (this != &other) {
        type_ = other.type_;
        value_ = other.value_;
        list_ = other.list_;
    }
    return *this;
}

// Arithmetic operators
Value Value::operator+(const Value& other) const {
    if (is_value() && other.is_value())
        return type_ == Type::Float || other.type_ == Type::Float
                   ? Value(value_ + other.value_)
                   : Value(static_cast<int>(value_ + other.value_));

    if (type_ == Type::List && other.type_ == Type::List) {
        std::vector<Value> result = list_;
        result.insert(result.end(), other.list_.begin(), other.list_.end());
        return Value(result);
    }

    if (type_ == Type::String || other.type_ == Type::String)
        return Value((to_string() + other.to_string()).c_str());

    throw std::runtime_error("Cannot add these types");
}

Value Value::operator-(const Value& other) const {
    if (!is_value() || !other.is_value())
        throw std::runtime_error("Cannot subtract non-value types");
    if (type_ == Type::Float || other.type_ == Type::Float)
        return Value(value_ - other.value_);
    return Value(static_cast<int>(value_ - other.value_));
}

Value Value::operator-() const { return Value(0) - *this; }

Value Value::operator*(const Value& other) const {
    if (!is_value() || !other.is_value())
        throw std::runtime_error("Cannot multiply non-value types");
    if (type_ == Type::Float || other.type_ == Type::Float)
        return Value(value_ * other.value_);
    return Value(static_cast<int>(value_ * other.value_));
}

Value Value::operator/(const Value& other) const {
    if (!is_value() || !other.is_value())
        throw std::runtime_error("Cannot divide non-value types");
    if (other.value_ == 0.0) throw std::runtime_error("Division by zero");
    if (type_ == Type::Float || other.type_ == Type::Float)
        return Value(value_ / other.value_);
    return Value(static_cast<int>(value_ / other.value_));
}

Value Value::operator%(const Value& other) const {
    if (!is_value() || !other.is_value())
        throw std::runtime_error("Cannot use modulo with non-value types");
    if (other.value_ == 0.0)
        throw std::runtime_error("Division by zero in modulo operation");

    double dividend = value_;
    double divisor = other.value_;

    double quotient = dividend / divisor;
    long long int_quotient = static_cast<long long>(quotient);

    double result = dividend - (divisor * int_quotient);

    if (type_ == Type::Float || other.type_ == Type::Float)
        return Value(result);
    return Value(static_cast<int>(result));
}

// Compound assignment operators
Value& Value::operator+=(const Value& other) {
    *this = *this + other;
    return *this;
}

Value& Value::operator-=(const Value& other) {
    *this = *this - other;
    return *this;
}

Value& Value::operator*=(const Value& other) {
    *this = *this * other;
    return *this;
}

Value& Value::operator/=(const Value& other) {
    *this = *this / other;
    return *this;
}

// Comparison operators
bool Value::operator==(const Value& other) const {
    if (is_value() && other.is_value()) return value_ == other.value_;
    if (is_iterable() && other.is_iterable()) return list_ == other.list_;
    return type_ == Type::Null && other.type_ == Type::Null;
}

bool Value::operator!=(const Value& other) const { return !(*this == other); }

bool Value::operator<(const Value& other) const {
    if (is_value() && other.is_value()) return value_ < other.value_;
    if (type_ == Type::String && other.type_ == Type::String)
        return to_string() < other.to_string();
    throw std::runtime_error("Cannot compare these types");
}

bool Value::operator>(const Value& other) const { return other < *this; }
bool Value::operator<=(const Value& other) const { return !(*this > other); }
bool Value::operator>=(const Value& other) const { return !(*this < other); }

// Logical operators
Value Value::operator&&(const Value& other) const {
    return Value((bool)*this && (bool)other);
}

Value Value::operator||(const Value& other) const {
    return Value((bool)*this || (bool)other);
}

Value Value::operator!() const { return Value(!(bool)*this); }

// Increment/Decrement operators
Value& Value::operator++() {
    if (!is_value())
        throw std::runtime_error("Cannot increment non-value type");
    *this = *this + Value(1);
    return *this;
}

Value Value::operator++(int) {
    Value temp(*this);
    ++(*this);
    return temp;
}

Value& Value::operator--() {
    if (!is_value())
        throw std::runtime_error("Cannot decrement non-value type");
    *this = *this - Value(1);
    return *this;
}

Value Value::operator--(int) {
    Value temp(*this);
    --(*this);
    return temp;
}

// Indexing operations
Value& Value::operator[](Value index) {
    if (index.type_ == Type::String) {
        if (fields_.find(index.to_string()) != fields_.end())
            return fields_.at(index.to_string());
        fields_[index.to_string()] = Value(nullptr);
        return fields_.at(index.to_string());
    }

    if (!is_iterable())
        throw std::runtime_error("Cannot index non-iterable type");
    if (index.type_ != Type::Int)
        throw std::runtime_error("Index must be an integer");
    if (index.value_ >= list_.size())
        throw std::out_of_range("Index out of range");
    return list_[static_cast<size_t>(index.value_)];
}

Value Value::operator[](Value index) const {
    if (index.type_ == Type::String && type_ == Type::Object) {
        if (fields_.find(index.to_string()) != fields_.end())
            return fields_.at(index.to_string());
        return Value(nullptr);
    }

    if (!is_iterable())
        throw std::runtime_error("Cannot index non-iterable type");
    if (index.type_ != Type::Int)
        throw std::runtime_error("Index must be an integer");
    if (index.value_ >= list_.size())
        throw std::out_of_range("Index out of range");
    return list_[static_cast<size_t>(index.value_)];
}

// Iterator support
std::vector<Value>::const_iterator Value::begin() const {
    if (!is_iterable())
        throw std::runtime_error("Cannot iterate non-iterable type");
    return list_.begin();
}

std::vector<Value>::const_iterator Value::end() const {
    if (!is_iterable())
        throw std::runtime_error("Cannot iterate non-iterable type");
    return list_.end();
}

// Conversions
std::string Value::to_string() const {
    switch (type_) {
        case Type::Null:
            return "null";
        case Type::Bool:
            return (value_ == 0.0 ? "false" : "true");
        case Type::Char:
            return std::string(1, static_cast<char>(value_));
        case Type::Int:
            return std::to_string(static_cast<int>(value_));
        case Type::Float:
            return std::to_string(value_);
        case Type::String: {
            std::ostringstream oss;
            for (const Value& char_var : list_)
                oss << static_cast<char>(char_var.value_);
            return oss.str();
        }
        case Type::List: {
            std::ostringstream oss;
            oss << "[";
            for (size_t i = 0; i < list_.size(); ++i) {
                if (i > 0) oss << ", ";
                oss << list_[i].to_string();
            }
            oss << "]";
            return oss.str();
        }
        case Type::Function:
            return "function";
        case Type::Object:
            return "object";
    }

    return "";
}

Value::operator bool() const {
    return (is_value() && value_ != 0.0) || (is_iterable() && !list_.empty());
}

// Stream operator
std::ostream& operator<<(std::ostream& os, const Value& value) {
    os << value.to_string();
    return os;
}

std::string Value::type() const {
    switch (type_) {
        case Type::Null:
            return "null";
        case Type::Int:
            return "int";
        case Type::Float:
            return "float";
        case Type::Bool:
            return "bool";
        case Type::Char:
            return "char";
        case Type::String:
            return "string";
        case Type::List:
            return "list";
        case Type::Function:
            return "function";
        case Type::Object:
            return "object";
    }
    return "unknown";
}

Value _type(const Value& value) { return Value(value.type().c_str()); }

Value _len(const Value& value) {
    if (value.type() == "list" || value.type() == "string")
        return Value(static_cast<int>(value.get_list().size()));
    throw std::runtime_error("Cannot get length of non-list type");
}

Value _null(const Value&) { return Value(nullptr); }

Value _int(const Value& value) {
    if (value.type() == "list" || value.type() == "string")
        throw std::runtime_error("Cannot convert non-value type to int");
    return Value(static_cast<int>(value.get_value()));
}

Value _float(const Value& value) {
    if (value.type() == "list" || value.type() == "string")
        throw std::runtime_error("Cannot convert non-value type to float");
    return Value(value.get_value());
}

Value _bool(const Value& value) { return (bool)value; }

Value _char(const Value& value) {
    if (value.type() == "list" || value.type() == "string")
        throw std::runtime_error("Cannot convert non-value type to char");
    return Value(static_cast<char>(value.get_value()));
}

Value _string(const Value& value) { return Value(value.to_string().c_str()); }

Value _list(const Value& value) {
    if (value.type() == "list") return value;
    if (value.type() == "string") return Value(value.get_list());
    return Value(std::vector<Value>{value});
}