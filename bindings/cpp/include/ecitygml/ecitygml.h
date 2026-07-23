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

typedef struct CCityModelArena CCityModelArena;

typedef struct CEnvelope CEnvelope;

typedef struct CGmlReader CGmlReader;

enum CErrorCode city_model_destroy(struct CCityModel *handle);

enum CErrorCode city_model_arena_create(struct CCityModel *city_model,
                                        struct CCityModelArena **out);

enum CErrorCode city_model_arena_destroy(struct CCityModelArena *handle);

enum CErrorCode city_model_arena_objects_len(struct CCityModelArena *handle, uintptr_t *out);

enum CErrorCode city_model_arena_get_object_ids(struct CCityModelArena *handle,
                                                char ***out_ptr,
                                                uintptr_t *out_len);

enum CErrorCode city_model_arena_free_object_ids(char **ptr, uintptr_t len);

enum CErrorCode city_model_arena_get_envelope(struct CCityModelArena *handle,
                                              const char *id,
                                              struct CEnvelope **out);

enum CErrorCode gml_reader_create(const char *file_path, struct CGmlReader **out);

enum CErrorCode gml_reader_destroy(struct CGmlReader *handle);

enum CErrorCode gml_reader_finish(struct CGmlReader *handle, struct CCityModel **out);

enum CErrorCode city_model_objects_len(struct CCityModel *handle, uintptr_t *out);

enum CErrorCode envelope_size_x(struct CEnvelope *handle, double *out);

enum CErrorCode envelope_size_y(struct CEnvelope *handle, double *out);

enum CErrorCode envelope_size_z(struct CEnvelope *handle, double *out);

enum CErrorCode envelope_destroy(struct CEnvelope *handle);

#endif  /* ECITYGML_H */
