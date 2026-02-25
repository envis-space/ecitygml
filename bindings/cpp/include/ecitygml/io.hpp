#pragma once
#include <iostream>

extern "C" {
    #include "./ecitygml.h"
}
#include "core.hpp"

class GmlReader
{
public:
    explicit GmlReader(const std::string& file_path)
    {
        CErrorCode err = gml_reader_create(file_path.c_str(), &handle_);
        if (err != CErrorCode::OK) {
            throw std::invalid_argument("Error code: " + std::to_string(static_cast<int>(err)));
        }

    };

    [[nodiscard]] CityModel finish() const
    {
        CCityModel* model = nullptr;
        CErrorCode err = gml_reader_finish(handle_, &model);
        if (err != CErrorCode::OK) {
            throw std::invalid_argument("Error code: " + std::to_string(static_cast<int>(err)));
        }
        auto* city_model_handle = new CityModel(model);

        return *city_model_handle;
    }

    ~GmlReader() { cleanup(); }

private:
    CGmlReader* handle_ = nullptr;

    void cleanup() noexcept {
        if (handle_) { gml_reader_destroy(handle_); handle_ = nullptr; }
    }
};
