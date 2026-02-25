#ifndef ECITYGML_H
#define ECITYGML_H

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum CErrorCode {
  OK = 0,
  NULL_POINTER = 1,
  INTERNAL_ERROR = 255,
} CErrorCode;

typedef struct CCityModel CCityModel;

typedef struct CCityModelGeometryIndex CCityModelGeometryIndex;

typedef struct CCityObjectGeometry CCityObjectGeometry;

typedef struct CEnvelope CEnvelope;

typedef struct CGmlReader CGmlReader;

enum CErrorCode city_model_destroy(struct CCityModel *handle);

enum CErrorCode city_model_geometry_index_create(struct CCityModel *city_model,
                                                 struct CCityModelGeometryIndex **out);

enum CErrorCode city_model_geometry_index_destroy(struct CCityModelGeometryIndex *handle);

enum CErrorCode city_model_geometry_index_objects_len(struct CCityModelGeometryIndex *handle,
                                                      uintptr_t *out);

enum CErrorCode city_model_geometry_index_get_object_ids(struct CCityModelGeometryIndex *handle,
                                                         char ***out_ptr,
                                                         uintptr_t *out_len);

enum CErrorCode city_model_geometry_index_free_object_ids(char **ptr, uintptr_t len);

enum CErrorCode city_model_geometry_index_get(struct CCityModelGeometryIndex *handle,
                                              const char *id,
                                              struct CCityObjectGeometry **out);

enum CErrorCode city_object_geometry_envelope(struct CCityObjectGeometry *handle,
                                              struct CEnvelope **out);

enum CErrorCode city_object_geometry_destroy(struct CCityObjectGeometry *handle);

enum CErrorCode gml_reader_create(const char *file_path, struct CGmlReader **out);

enum CErrorCode gml_reader_destroy(struct CGmlReader *handle);

enum CErrorCode gml_reader_finish(struct CGmlReader *handle, struct CCityModel **out);

enum CErrorCode city_model_objects_len(struct CCityModel *handle, uintptr_t *out);

enum CErrorCode envelope_size_x(struct CEnvelope *handle, double *out);

enum CErrorCode envelope_size_y(struct CEnvelope *handle, double *out);

enum CErrorCode envelope_size_z(struct CEnvelope *handle, double *out);

enum CErrorCode envelope_destroy(struct CEnvelope *handle);

#endif  /* ECITYGML_H */
