#ifndef STD_HPP
#define STD_HPP

#include <iostream>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>

enum class Type { NONE, INT, FLOAT, BOOL, CHAR, STRING, LIST };

class Value {
   private:
    Type type_;
    double numeric_value_;
    std::vector<Value> list_value_;

    bool is_list() const { return type_ == Type::LIST; }
    bool is_string() const { return type_ == Type::STRING; }
    bool is_iterable() const { return is_list() || is_string(); }
    bool is_int() const { return type_ == Type::INT; }
    bool is_float() const { return type_ == Type::FLOAT; }
    bool is_value() const {
        return type_ == Type::INT || type_ == Type::FLOAT ||
               type_ == Type::BOOL || type_ == Type::CHAR;
    }
    bool is_null() const { return type_ == Type::NONE; }

   public:
    Value(const Value& other);
    Value(std::nullptr_t);
    Value(int val);
    Value(double val);
    Value(bool val);
    Value(char val);
    Value(const char* val);
    Value(const std::vector<Value>& val);

    explicit operator bool() const;

    Value& operator=(const Value& other);

    Value operator+(const Value& other) const;
    Value operator-(const Value& other) const;
    Value operator-() const;
    Value operator*(const Value& other) const;
    Value operator/(const Value& other) const;
    Value operator%(const Value& other) const;

    Value& operator+=(const Value& other);
    Value& operator-=(const Value& other);
    Value& operator*=(const Value& other);
    Value& operator/=(const Value& other);

    bool operator==(const Value& other) const;
    bool operator!=(const Value& other) const;
    bool operator<(const Value& other) const;
    bool operator>(const Value& other) const;
    bool operator<=(const Value& other) const;
    bool operator>=(const Value& other) const;

    Value operator&&(const Value& other) const;
    Value operator||(const Value& other) const;
    Value operator!() const;

    Value& operator++();
    Value operator++(int);
    Value& operator--();
    Value operator--(int);

    Value& operator[](Value index);
    const Value& operator[](Value index) const;

    std::vector<Value>::const_iterator begin() const;
    std::vector<Value>::const_iterator end() const;

    friend std::ostream& operator<<(std::ostream& os, const Value& var);

    std::string to_string() const;

    Type type() const { return type_; }
    double get_numeric_value() const { return numeric_value_; }
    const std::vector<Value>& get_list_value() const { return list_value_; }
};

template <typename... Args>
Value _print(const Args&... args) {
    bool first = true;
    ((std::cout << (first ? (first = false, "") : " ") << args), ...);
    std::cout << std::endl;
    return Value(nullptr);
}

Value _type(const Value& value);

Value _len(const Value& value);

Value _null(const Value&);

Value _int(const Value& value);

Value _float(const Value& value);

Value _bool(const Value& value);

Value _char(const Value& value);

Value _string(const Value& value);

Value _list(const Value& value);

#endif