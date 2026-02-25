#include <catch2/catch_test_macros.hpp>
#include <ecitygml/io.hpp>

#include <catch2/catch_all.hpp>
#include "catch2/matchers/catch_matchers.hpp"

TEST_CASE("CitygmlReader construction", "[reader]")
{
    REQUIRE_NOTHROW(GmlReader("./tests/fixtures/FZK-Haus-LoD1-KIT-IAI-KHH-B36-V1__v3.gml"));
}

TEST_CASE("CitygmlReader finish", "[reader]") {
    const GmlReader reader("./tests/fixtures/FZK-Haus-LoD1-KIT-IAI-KHH-B36-V1__v3.gml");
    REQUIRE_NOTHROW(reader.finish());
}

TEST_CASE("CitygmlReader model", "[reader]") {
    const GmlReader reader("./tests/fixtures/FZK-Haus-LoD1-KIT-IAI-KHH-B36-V1__v3.gml");
    auto city_model = reader.finish();

    REQUIRE(city_model.objects_len() == 1);
}

TEST_CASE("GeometryCollector", "[reader]") {
    const GmlReader reader("./tests/fixtures/FZK-Haus-LoD2-KIT-IAI-KHH-B36-V1__v3.gml");
    auto city_model = reader.finish();
    const auto city_model_geometry_index = CityModelGeometryIndex(&city_model);

    REQUIRE(city_model_geometry_index.objects_len() == 8);
}

TEST_CASE("GeometryCollector", "[get]") {
    const GmlReader reader("./tests/fixtures/FZK-Haus-LoD2-KIT-IAI-KHH-B36-V1__v3.gml");
    auto city_model = reader.finish();
    const auto city_model_geometry_index = CityModelGeometryIndex(&city_model);

    const auto city_object_geometry = city_model_geometry_index.get("GML_5856d7ad-5e34-498a-817b-9544bfbb1475");

    const auto envelope = city_object_geometry.envelope();
    REQUIRE_THAT(envelope.size_x(), Catch::Matchers::WithinRel(0.0, 0.001));
    REQUIRE_THAT(envelope.size_y(), Catch::Matchers::WithinRel(10.0, 0.001));
    REQUIRE_THAT(envelope.size_z(), Catch::Matchers::WithinRel(6.51769, 0.001));
}

TEST_CASE("GeometryCollector", "[iterator]") {
    const GmlReader reader("./tests/fixtures/FZK-Haus-LoD2-KIT-IAI-KHH-B36-V1__v3.gml");
    auto city_model = reader.finish();
    const auto city_model_geometry_index = CityModelGeometryIndex(&city_model);

    auto ids = city_model_geometry_index.get_object_ids();
    for (size_t i = 0; i < ids.size(); ++i) {
        std::cout << ids[i] << std::endl;
    }


}
