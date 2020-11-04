#include "VHACD.h"

#ifndef VHACD_BRIDGE_H
#define VHACD_BRIDGE_H

void IVHACD_Cancel(VHACD::IVHACD* self);

bool IVHACD_Compute_f32(
    VHACD::IVHACD* self,
    const float* const points,
    const uint32_t countPoints,
    const uint32_t* const triangles,
    const uint32_t countTriangles,
    const VHACD::IVHACD::Parameters& params);

bool IVHACD_Compute_f64(
    VHACD::IVHACD* self,
    const double* const points,
    const uint32_t countPoints,
    const uint32_t* const triangles,
    const uint32_t countTriangles,
    const VHACD::IVHACD::Parameters& params);

uint32_t IVHACD_GetNConvexHulls(const VHACD::IVHACD* self);

void IVHACD_GetConvexHull(
    const VHACD::IVHACD* self,
    const uint32_t index,
    VHACD::IVHACD::ConvexHull& ch);

void IVHACD_Clean(VHACD::IVHACD* self); // release internally allocated memory

void IVHACD_Release(VHACD::IVHACD* self);

#if 0
bool IVHACD_OCLInit(
    VHACD::IVHACD* self,
    void* const oclDevice,
    VHACD::IVHACD::IUserLogger* const logger = 0);

bool IVHACD_OCLRelease(
    VHACD::IVHACD* self,
    VHACD::IVHACD::IUserLogger* const logger = 0);
#endif

// Will compute the center of mass of the convex hull decomposition results and return it
// in 'centerOfMass'.  Returns false if the center of mass could not be computed.
bool IVHACD_ComputeCenterOfMass(
    const VHACD::IVHACD* self,
    double centerOfMass[3]);

// In synchronous mode (non-multi-threaded) the state is always 'ready'
// In asynchronous mode, this returns true if the background thread is not still actively computing
// a new solution.  In an asynchronous config the 'IsReady' call will report any update or log
// messages in the caller's current thread.
bool IVHACD_IsReady_typed(const VHACD::IVHACD* self);

typedef void (*UserCallback)(
    const double overallProgress,
    const double stageProgress,
    const double operationProgress,
    const char* const stage,
    const char* const operation);
VHACD::IVHACD::IUserCallback* IVHACD_CreateUserCallback(UserCallback callback);
void IVHACD_FreeUserCallback(VHACD::IVHACD::IUserCallback* callback);

typedef void (*UserLogger)(
    const char* const msg);
VHACD::IVHACD::IUserLogger* IVHACD_CreateUserLogger(UserLogger logger);
void IVHACD_FreeUserLogger(VHACD::IVHACD::IUserLogger* logger);

#endif // VHACD_BRIDGE_H