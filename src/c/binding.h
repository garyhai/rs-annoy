#pragma once

#include "annoylib.h"

#ifdef __cplusplus
extern "C" {
#endif

  AnnoyIndexInterface<int32_t, float>* annoy_index_angular(int f);

  void annoy_delete_index(AnnoyIndexInterface<int32_t, float> *index);

  bool annoy_add_item(AnnoyIndexInterface<int32_t, float> *index, int item, float *w);

  bool annoy_build(AnnoyIndexInterface<int32_t, float> *index, int q);
  
  void annoy_get_item(AnnoyIndexInterface<int32_t, float> *index, int item, float *result);
  
  void annoy_get_nns_by_item(AnnoyIndexInterface<int32_t, float> *index, int item, int n, int search_k, int *result, float *distances);
  
  void annoy_get_nns_by_vector(AnnoyIndexInterface<int32_t, float> *index, const float *w, int n, int search_k, int *result, float *distances);

  bool annoy_load(AnnoyIndexInterface<int32_t, float> *index, const char *file);

  bool annoy_save(AnnoyIndexInterface<int32_t, float> *index, const char *file);
  
#ifdef __cplusplus
}
#endif