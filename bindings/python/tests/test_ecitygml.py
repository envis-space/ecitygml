from ecitygml import GmlReader

def test_reading_tum_main_entrance() -> None:
    reader = GmlReader("../../tests/fixtures/TUM-Main-Entrance.gml")
    city_model = reader.finish()
    assert city_model.city_objects_len() == 1

def test_reading_fzk_haus_lod2() -> None:
    reader = GmlReader("../../tests/fixtures/FZK-Haus-LoD2-KIT-IAI-KHH-B36-V1__v3.gml")
    city_model = reader.finish()
    assert city_model.city_objects_len() == 1

def test_reading_asam_ex_bidirectional() -> None:
    reader = GmlReader("../../tests/fixtures/ASAM-Ex_Bidirectional_Junction.gml")
    city_model = reader.finish()
    assert city_model.city_objects_len() == 1
