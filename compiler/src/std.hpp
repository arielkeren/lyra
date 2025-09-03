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
    Value(const Value& other);
    Value(std::nullptr_t);
    Value(int val);
    Value(double val);
    Value(bool val);
    Value(char val);
    Value(const char* val);
    Value(const std::vector<Value>& val);
    Value() : type_(Type::Null), value_(0.0) {}

    Value(std::function<Value(const std::vector<Value>&)> func)
        : type_(Type::Function), value_(0.0), function_(func) {}
    template <typename... Args>
    Value operator()(const Args&... args) const {
        if (type_ != Type::Function || !function_) {
            throw std::runtime_error("Value is not callable");
        }
        std::vector<Value> arg_vector = {Value(args)...};
        return function_(arg_vector);
    }
    Value operator()() const {
        if (type_ != Type::Function || !function_) {
            throw std::runtime_error("Value is not callable");
        }
        return function_({});
    }
    void set_field(const std::string& name, const Value& value) {
        if (type_ != Type::Object) {
            throw std::runtime_error("Value is not an object");
        }
        fields_[name] = value;
    }

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
    Value operator[](Value index) const;

    std::vector<Value>::const_iterator begin() const;
    std::vector<Value>::const_iterator end() const;

    friend std::ostream& operator<<(std::ostream& os, const Value& var);

    std::string to_string() const;

    std::string type() const;
    double get_value() const { return value_; }
    const std::vector<Value>& get_list() const { return list_; }

    static void register_method(
        const std::string& name,
        std::function<Value(const std::vector<Value>&)> method) {
        global_methods_[name] = method;
    }

    Value operator[](const char* method_name) {
        if (global_methods_.find(method_name) == global_methods_.end()) {
            throw std::runtime_error("Method '" + std::string(method_name) +
                                     "' not found");
        }

        Value callable_method(
            [this, method_name](const std::vector<Value>& args) -> Value {
                std::vector<Value> method_args = {*this};
                method_args.insert(method_args.end(), args.begin(), args.end());
                return global_methods_[method_name](method_args);
            });

        return callable_method;
    }

    Value operator[](const char* method_name) const {
        if (global_methods_.find(method_name) == global_methods_.end()) {
            throw std::runtime_error("Method '" + std::string(method_name) +
                                     "' not found");
        }

        Value callable_method(
            [this, method_name](const std::vector<Value>& args) -> Value {
                std::vector<Value> method_args = {*this};
                method_args.insert(method_args.end(), args.begin(), args.end());
                return global_methods_[method_name](method_args);
            });

        return callable_method;
    }
};

class Range {
   private:
    int start_;
    int end_;

   public:
    Range(Value start, Value end) {
        if (start.type() != "int" || end.type() != "int")
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