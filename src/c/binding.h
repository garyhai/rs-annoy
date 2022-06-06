#pragma once

#include "annoylib.h"

#ifdef __cplusplus
using namespace Annoy;
extern "C" {
#endif

  AnnoyIndexInterface<uint32_t, float>* annoy_index_angular(int f);

  void annoy_delete_index(AnnoyIndexInterface<uint32_t, float> *index);

  bool annoy_add_item(AnnoyIndexInterface<uint32_t, float> *index, uint32_t item, float *w);

  bool annoy_build(AnnoyIndexInterface<uint32_t, float> *index, int q);
  
  bool annoy_get_item(AnnoyIndexInterface<uint32_t, float> *index, uint32_t item, float *result);

  uint32_t annoy_get_n_items(AnnoyIndexInterface<uint32_t, float> *index);
 
  size_t annoy_get_nns_by_item(AnnoyIndexInterface<uint32_t, float> *index, uint32_t item, size_t n, int search_k, uint32_t *result, float *distances);
  
  size_t annoy_get_nns_by_vector(AnnoyIndexInterface<uint32_t, float> *index, const float *w, size_t n, int search_k, uint32_t *result, float *distances);

  bool annoy_load(AnnoyIndexInterface<uint32_t, float> *index, const char *file);
  
  void annoy_unload(AnnoyIndexInterface<uint32_t, float> *index);

  bool annoy_save(AnnoyIndexInterface<uint32_t, float> *index, const char *file);
  
#ifdef __cplusplus
}
#endif