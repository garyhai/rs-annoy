#include "binding.h"
#include "annoylib.h"
#include "kissrandom.h"

using namespace std;

AnnoyIndexInterface<uint32_t, float> *annoy_index_angular(int f)
{
    return new ::AnnoyIndex<uint32_t, float, ::Angular, ::Kiss64Random, AnnoyIndexSingleThreadedBuildPolicy>(f);
}

bool annoy_save(AnnoyIndexInterface<uint32_t, float> *index, const char *file)
{
    return index->save(file, false);
}

bool annoy_load(AnnoyIndexInterface<uint32_t, float> *index, const char *file)
{
    return index->load(file, false);
}

void annoy_unload(AnnoyIndexInterface<uint32_t, float> *index)
{
    index->unload();
}

void annoy_delete_index(AnnoyIndexInterface<uint32_t, float> *index)
{
    delete index;
}

bool annoy_add_item(AnnoyIndexInterface<uint32_t, float> *index, uint32_t item, float *w)
{
    return index->add_item(item, w);
}

bool annoy_build(AnnoyIndexInterface<uint32_t, float> *index, int q)
{
    return index->build(q);
}

bool annoy_on_disk_build(AnnoyIndexInterface<uint32_t, float> *index, const char *file)
{
    return index->on_disk_build(file);
}


bool annoy_get_item(
    AnnoyIndexInterface<uint32_t, float> *index, 
    uint32_t item, 
    float *result
) {
    if (item < index->get_n_items()) {
        index->get_item(item, result);
        return true;
    }
    return false;
}

size_t annoy_get_nns_by_item(
    AnnoyIndexInterface<uint32_t, float> *index, 
    uint32_t item, 
    size_t n, 
    int search_k, 
    uint32_t *result, 
    float *distances
) {
    std::vector<uint32_t> resultV;
    std::vector<float> distancesV;

    index->get_nns_by_item(item, n, search_k, &resultV, &distancesV);

    std::copy(resultV.begin(), resultV.end(), result);
    std::copy(distancesV.begin(), distancesV.end(), distances);
    return resultV.size();
}

size_t annoy_get_nns_by_vector(
    AnnoyIndexInterface<uint32_t, float> *index, 
    const float *w, 
    size_t n, 
    int search_k, 
    uint32_t *result, 
    float *distances
) {
    std::vector<uint32_t> resultV;
    std::vector<float> distancesV;

    index->get_nns_by_vector(w, n, search_k, &resultV, &distancesV);

    std::copy(resultV.begin(), resultV.end(), result);
    std::copy(distancesV.begin(), distancesV.end(), distances);
    return resultV.size();
}

uint32_t annoy_get_n_items(AnnoyIndexInterface<uint32_t, float> *index) {
    return index->get_n_items();
}