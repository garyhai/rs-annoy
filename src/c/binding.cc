#include "binding.h"
#include "annoylib.h"
#include "kissrandom.h"

using namespace std;

AnnoyIndexInterface<int32_t, float> *annoy_index_angular(int f)
{
    return new ::AnnoyIndex<int32_t, float, ::Angular, ::Kiss64Random, AnnoyIndexSingleThreadedBuildPolicy>(f);
}

bool annoy_save(AnnoyIndexInterface<int32_t, float> *index, const char *file)
{
    return index->save(file, false);
}

bool annoy_load(AnnoyIndexInterface<int32_t, float> *index, const char *file)
{
    return index->load(file, false);
}

void annoy_unload(AnnoyIndexInterface<int32_t, float> *index)
{
    index->unload();
}

void annoy_delete_index(AnnoyIndexInterface<int32_t, float> *index)
{
    delete index;
}

bool annoy_add_item(AnnoyIndexInterface<int32_t, float> *index, int item, float *w)
{
    return index->add_item(item, w);
}

bool annoy_build(AnnoyIndexInterface<int32_t, float> *index, int q)
{
    return index->build(q);
}

void annoy_get_item(
    AnnoyIndexInterface<int32_t, float> *index, 
    int item, 
    float *result
) {
    index->get_item(item, result);
}

size_t annoy_get_nns_by_item(
    AnnoyIndexInterface<int32_t, float> *index, 
    int item, 
    int n, 
    int search_k, 
    int *result, 
    float *distances
) {
    std::vector<int32_t> resultV;
    std::vector<float> distancesV;

    index->get_nns_by_item(item, n, search_k, &resultV, &distancesV);

    std::copy(resultV.begin(), resultV.end(), result);
    std::copy(distancesV.begin(), distancesV.end(), distances);
    return resultV.size();
}

size_t annoy_get_nns_by_vector(
    AnnoyIndexInterface<int32_t, float> *index, 
    const float *w, 
    int n, 
    int search_k, 
    int *result, 
    float *distances
) {
    std::vector<int32_t> resultV;
    std::vector<float> distancesV;

    index->get_nns_by_vector(w, n, search_k, &resultV, &distancesV);

    std::copy(resultV.begin(), resultV.end(), result);
    std::copy(distancesV.begin(), distancesV.end(), distances);
    return resultV.size();
}
