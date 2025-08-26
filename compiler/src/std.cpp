#include "std.hpp"

#include <cmath>
#include <sstream>

// Constructors
Value::Value(std::nullptr_t) : type_(Type::NONE) {}

Value::Value(int value)
    : type_(Type::INT), numeric_value_(static_cast<double>(value)) {}

Value::Value(double value) : type_(Type::FLOAT), numeric_value_(value) {}

Value::Value(bool value)
    : type_(Type::BOOL), numeric_value_(value ? 1.0 : 0.0) {}

Value::Value(char value)
    : type_(Type::CHAR), numeric_value_(static_cast<double>(value)) {}

Value::Value(const char* value) : type_(Type::STRING) {
    list_value_.reserve(strlen(value));
    for (const char* p = value; *p != '\0'; ++p)
        list_value_.push_back(Value(*p));
}

Value::Value(const std::vector<Value>& value)
    : type_(Type::LIST), list_value_(value) {}

// Assignment operators
Value& Value::operator=(const Value& other) {
    if (this != &other) {
        type_ = other.type_;
        numeric_value_ = other.numeric_value_;
        list_value_ = other.list_value_;
    }
    return *this;
}

// Arithmetic operators
Value Value::operator+(const Value& other) const {
    if (is_value() && other.is_value())
        return this->is_float() || other.is_float()
                   ? Value(numeric_value_ + other.numeric_value_)
                   : Value(static_cast<int>(numeric_value_ +
                                            other.numeric_value_));

    if (is_list() && other.is_list()) {
        std::vector<Value> result = list_value_;
        result.insert(result.end(), other.list_value_.begin(),
                      other.list_value_.end());
        return Value(result);
    }

    if (is_string() || other.is_string())
        return Value((to_string() + other.to_string()).c_str());

    throw std::runtime_error("Cannot add these types");
}

Value Value::operator-(const Value& other) const {
    if (!is_value() || !other.is_value())
        throw std::runtime_error("Cannot subtract non-value types");
    if (is_float() || other.is_float())
        return Value(numeric_value_ - other.numeric_value_);
    return Value(static_cast<int>(numeric_value_ - other.numeric_value_));
}

Value Value::operator*(const Value& other) const {
    if (!is_value() || !other.is_value())
        throw std::runtime_error("Cannot multiply non-value types");
    if (is_float() || other.is_float())
        return Value(numeric_value_ * other.numeric_value_);
    return Value(static_cast<int>(numeric_value_ * other.numeric_value_));
}

Value Value::operator/(const Value& other) const {
    if (!is_value() || !other.is_value())
        throw std::runtime_error("Cannot divide non-value types");
    if (other.numeric_value_ == 0.0)
        throw std::runtime_error("Division by zero");
    if (is_float() || other.is_float())
        return Value(numeric_value_ / other.numeric_value_);
    return Value(static_cast<int>(numeric_value_ / other.numeric_value_));
}

Value Value::operator%(const Value& other) const {
    if (!is_value() || !other.is_value())
        throw std::runtime_error("Cannot use modulo with non-value types");
    if (other.numeric_value_ == 0.0)
        throw std::runtime_error("Division by zero in modulo operation");

    double dividend = numeric_value_;
    double divisor = other.numeric_value_;

    double quotient = dividend / divisor;
    long long int_quotient = static_cast<long long>(quotient);

    double result = dividend - (divisor * int_quotient);

    if (is_float() || other.is_float()) return Value(result);
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
    if (is_value() && other.is_value())
        return numeric_value_ == other.numeric_value_;
    if (is_iterable() && other.is_iterable())
        return list_value_ == other.list_value_;
    return is_null() && other.is_null();
}

bool Value::operator!=(const Value& other) const { return !(*this == other); }

bool Value::operator<(const Value& other) const {
    if (is_value() && other.is_value())
        return numeric_value_ < other.numeric_value_;
    if (is_string() && other.is_string())
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
Value& Value::operator[](size_t index) {
    if (!is_iterable()) {
        throw std::runtime_error("Cannot index non-iterable type");
    }
    if (index >= list_value_.size()) {
        throw std::out_of_range("Index out of range");
    }
    return list_value_[index];
}

const Value& Value::operator[](size_t index) const {
    if (!is_iterable())
        throw std::runtime_error("Cannot index non-iterable type");
    if (index >= list_value_.size())
        throw std::out_of_range("Index out of range");
    return list_value_[index];
}

// Iterator support
std::vector<Value>::const_iterator Value::begin() const {
    if (!is_iterable())
        throw std::runtime_error("Cannot iterate non-iterable type");
    return list_value_.begin();
}

std::vector<Value>::const_iterator Value::end() const {
    if (!is_iterable())
        throw std::runtime_error("Cannot iterate non-iterable type");
    return list_value_.end();
}

// Conversions
std::string Value::to_string() const {
    switch (type_) {
        case Type::NONE:
            return "null";
        case Type::INT:
            return std::to_string(static_cast<int>(numeric_value_));
        case Type::FLOAT:
            return std::to_string(numeric_value_);
        case Type::BOOL:
            return (numeric_value_ == 0.0 ? "false" : "true");
        case Type::STRING: {
            std::ostringstream oss;
            for (const Value& char_var : list_value_)
                oss << static_cast<char>(char_var.numeric_value_);
            return oss.str();
        }
        case Type::LIST: {
            std::ostringstream oss;
            oss << "[";
            for (size_t i = 0; i < list_value_.size(); ++i) {
                if (i > 0) oss << ", ";
                oss << list_value_[i].to_string();
            }
            oss << "]";
            return oss.str();
        }
    }
    return "";
}

Value::operator bool() const {
    return (is_value() && numeric_value_ != 0.0) ||
           (is_iterable() && !list_value_.empty());
}

// Stream operator
std::ostream& operator<<(std::ostream& os, const Value& value) {
    os << value.to_string();
    return os;
}

Value type(const Value& value) {
    switch (value.type()) {
        case Type::NONE:
            return Value("none");
        case Type::INT:
            return Value("int");
        case Type::FLOAT:
            return Value("float");
        case Type::BOOL:
            return Value("bool");
        case Type::CHAR:
            return Value("char");
        case Type::STRING:
            return Value("string");
        case Type::LIST:
            return Value("list");
    }
    return Value("");
}