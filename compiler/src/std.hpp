#ifndef STD_HPP
#define STD_HPP

#include <functional>
#include <iostream>
#include <sstream>
#include <stdexcept>
#include <string>
#include <unordered_map>
#include <vector>

enum class Type {
    Null,
    Bool,
    Char,
    Int,
    Float,
    String,
    List,
    Function,
    Object
};

class Value {
   private:
    Type type_;
    double value_;
    std::vector<Value> list_;
    std::function<Value(const std::vector<Value>&)> function_;
    std::unordered_map<std::string, Value> fields_;

    static std::unordered_map<std::string,
                              std::function<Value(const std::vector<Value>&)>>
        global_methods_;

    bool is_iterable() const {
        return type_ == Type::List || type_ == Type::String;
    }
    bool is_value() const {
        return type_ == Type::Int || type_ == Type::Float ||
               type_ == Type::Bool || type_ == Type::Char;
    }

   public:
    Value();
    Value(const Value& other);
    Value(std::nullptr_t);
    Value(bool val);
    Value(char val);
    Value(int val);
    Value(double val);
    Value(const char* val);
    Value(const std::vector<Value>& val);
    Value(std::function<Value(const std::vector<Value>&)> func);

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
    Value& operator%=(const Value& other);

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
    Value operator[](Value index) const;
    Value operator[](const char* method_name);
    Value operator[](const char* method_name) const;

    Value operator()() const;

    friend std::ostream& operator<<(std::ostream& os, const Value& var);

    std::vector<Value>::const_iterator begin() const;
    std::vector<Value>::const_iterator end() const;

    std::string to_string() const;

    Type get_type() const;
    double get_value() const;
    const std::vector<Value>& get_list() const;

    void set_field(const std::string& name, const Value& value);

    static void register_method(
        const std::string& name,
        std::function<Value(const std::vector<Value>&)> method);

    template <typename... Args>
    Value operator()(const Args&... args) const {
        if (type_ != Type::Function || !function_) {
            throw std::runtime_error("Value is not callable");
        }
        std::vector<Value> arg_vector = {Value(args)...};
        return function_(arg_vector);
    }
};

class Range {
   private:
    int start_;
    int end_;

   public:
    Range(Value start, Value end) {
        if (start.get_type() != Type::Int || end.get_type() != Type::Int)
            throw std::runtime_error("Range start and end must be integers");
        start_ = start.get_value();
        end_ = end.get_value();
    }

    class iterator {
       private:
        int current_;
        int end_;

       public:
        iterator(int start, int end) : current_(start), end_(end) {}

        Value operator*() const { return Value(current_); }

        iterator& operator++() {
            ++current_;
            return *this;
        }

        bool operator!=(const iterator& other) const {
            return current_ != other.current_;
        }
    };

    iterator begin() const { return iterator(start_, end_); }
    iterator end() const { return iterator(end_, end_); }
};

Value _type(const Value& value);

Value _len(const Value& value);

Value _null(const Value&);

Value _int(const Value& value);

Value _float(const Value& value);

Value _bool(const Value& value);

Value _char(const Value& value);

Value _string(const Value& value);

Value _list(const Value& value);

template <typename... Args>
Value _print(const Args&... args) {
    bool first = true;
    ((std::cout << (first ? (first = false, "") : " ") << args), ...);
    std::cout << std::endl;
    return Value(nullptr);
}

template <typename... Args>
void _exit(const Args&... args) {
    bool first = true;
    ((std::cerr << (first ? (first = false, "") : " ") << args), ...);
    std::cerr << std::endl;
    std::exit(1);
}

#endif