#pragma once

#include <string>
#include <stdexcept>
#include <cstdint>

extern "C" {
    #include "./ecitygml.h"
}

class Envelope
{
public:
    explicit Envelope(CEnvelope* handle) : handle_(handle)
    {};

    [[nodiscard]] double size_x() const
    {
        double value = 0;
        CErrorCode err = envelope_size_x(handle_, &value);
        if (err != CErrorCode::OK) {
            throw std::invalid_argument("Error code: " + std::to_string(static_cast<int>(err)));
        }

        return value;
    }

    [[nodiscard]] double size_y() const
    {
        double value = 0;
        CErrorCode err = envelope_size_y(handle_, &value);
        if (err != CErrorCode::OK) {
            throw std::invalid_argument("Error code: " + std::to_string(static_cast<int>(err)));
        }

        return value;
    }

    [[nodiscard]] double size_z() const
    {
        double value = 0;
        CErrorCode err = envelope_size_z(handle_, &value);
        if (err != CErrorCode::OK) {
            throw std::invalid_argument("Error code: " + std::to_string(static_cast<int>(err)));
        }

        return value;
    }

    [[nodiscard]] bool has_handle() const noexcept {
        return handle_ != nullptr;
    }

    ~Envelope() { cleanup(); }

private:
    CEnvelope* handle_ = nullptr;

    void cleanup() noexcept {
        if (handle_) { envelope_destroy(handle_); handle_ = nullptr; }
    }
};

class CityObjectGeometry
{
public:
    explicit CityObjectGeometry(CCityObjectGeometry* handle) : handle_(handle)
    {};

    [[nodiscard]] Envelope envelope() const
    {
        CEnvelope* c_envelope = nullptr;
        CErrorCode err = city_object_geometry_envelope(handle_, &c_envelope);
        if (err != CErrorCode::OK) {
            throw std::invalid_argument("Error code: " + std::to_string(static_cast<int>(err)));
        }

        Envelope envelope(c_envelope);
        return envelope;
    }

    ~CityObjectGeometry() { cleanup(); }

private:
    CCityObjectGeometry* handle_ = nullptr;

    void cleanup() noexcept {
        if (handle_) { city_object_geometry_destroy(handle_); handle_ = nullptr; }
    }
};


class CityModel
{
public:
    explicit CityModel(CCityModel* handle) : handle_(handle)
    {};

    [[nodiscard]] uint objects_len() const
    {
        uintptr_t count = 0;
        CErrorCode err = city_model_objects_len(handle_, &count);
        if (err != CErrorCode::OK) {
            throw std::invalid_argument("Error code: " + std::to_string(static_cast<int>(err)));
        }

        return count;
    }

    [[nodiscard]] CCityModel* handle() const noexcept { return handle_; }

    ~CityModel() { cleanup(); }

private:
    CCityModel* handle_ = nullptr;

    void cleanup() noexcept {
        if (handle_) { city_model_destroy(handle_); handle_ = nullptr; }
    }
};

class ObjectIds {
public:
    ObjectIds(char** ptr, size_t len)
        : ptr_(ptr), len_(len) {}

    ~ObjectIds() {
        if (ptr_ != nullptr) {
            city_model_geometry_index_free_object_ids(ptr_, len_);
        }
    }

    // Delete copy operations
    ObjectIds(const ObjectIds&) = delete;
    ObjectIds& operator=(const ObjectIds&) = delete;

    // Enable move operations
    ObjectIds(ObjectIds&& other) noexcept
        : ptr_(other.ptr_), len_(other.len_) {
        other.ptr_ = nullptr;
        other.len_ = 0;
    }

    ObjectIds& operator=(ObjectIds&& other) noexcept {
        if (this != &other) {
            // Clean up existing resources
            if (ptr_ != nullptr) {
                city_model_geometry_index_free_object_ids(ptr_, len_);
            }

            // Move from other
            ptr_ = other.ptr_;
            len_ = other.len_;

            // Reset other
            other.ptr_ = nullptr;
            other.len_ = 0;
        }
        return *this;
    }

    [[nodiscard]] size_t size() const { return len_; }

    [[nodiscard]] const char* operator[](size_t index) const {
        if (index >= len_) {
            throw std::out_of_range("Index out of range");
        }
        return ptr_[index];
    }

    // Iterator support for range-based for loops
    class Iterator {
    public:
        using iterator_category = std::random_access_iterator_tag;
        using value_type = const char*;
        using difference_type = std::ptrdiff_t;
        using pointer = const char**;
        using reference = const char*;

        Iterator(char** ptr) : ptr_(ptr) {}

        reference operator*() const { return *ptr_; }
        Iterator& operator++() { ++ptr_; return *this; }
        Iterator operator++(int) { Iterator tmp = *this; ++ptr_; return tmp; }
        bool operator==(const Iterator& other) const { return ptr_ == other.ptr_; }
        bool operator!=(const Iterator& other) const { return ptr_ != other.ptr_; }

    private:
        char** ptr_;
    };

    Iterator begin() const { return Iterator(ptr_); }
    Iterator end() const { return Iterator(ptr_ + len_); }

private:
    char** ptr_;
    size_t len_;
};


class CityModelGeometryIndex
{
public:
    explicit CityModelGeometryIndex(const CityModel* city_model)
    {
        CErrorCode err = city_model_geometry_index_create(city_model->handle(), &handle_);
        if (err != CErrorCode::OK) {
            throw std::invalid_argument("Error code: " + std::to_string(static_cast<int>(err)));
        }
    };

    [[nodiscard]] CCityModelGeometryIndex* handle() const noexcept { return handle_; }


    [[nodiscard]] uint objects_len() const
    {
        uintptr_t count = 0;
        CErrorCode err = city_model_geometry_index_objects_len(handle_, &count);
        if (err != CErrorCode::OK) {
            throw std::invalid_argument("Error code: " + std::to_string(static_cast<int>(err)));
        }

        return count;
    }

    [[nodiscard]] ObjectIds get_object_ids() const {
        char** ptr = nullptr;
        uintptr_t len = 0;

        CErrorCode err = city_model_geometry_index_get_object_ids(handle_, &ptr, &len);
        if (err != CErrorCode::OK) {
            throw std::runtime_error("Error code: " + std::to_string(static_cast<int>(err)));
        }

        return ObjectIds(ptr, len);
    }

    [[nodiscard]] CityObjectGeometry get(const std::string& id) const
    {
        CCityObjectGeometry* geometry_collection = nullptr;
        CErrorCode err = city_model_geometry_index_get(handle_, id.c_str(), &geometry_collection);
        if (err != CErrorCode::OK) {
            throw std::invalid_argument("Error code: " + std::to_string(static_cast<int>(err)));
        }

        CityObjectGeometry city_object_geometry_collection(geometry_collection);
        return city_object_geometry_collection;
    }

    ~CityModelGeometryIndex() { cleanup(); }

private:
    CCityModelGeometryIndex* handle_ = nullptr;

    void cleanup() noexcept {
        if (handle_) { city_model_geometry_index_destroy(handle_); handle_ = nullptr; }
    }
};
