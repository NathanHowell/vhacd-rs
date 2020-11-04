#include "VHACD.h"
#include "bridge.h"

// include these header only symbols in the static lib
void IVHACD_reference_header_only_symbols() {
    VHACD::IVHACD::Parameters p;
    p.Init();
}

void IVHACD_Cancel(VHACD::IVHACD* self) {
    self->Cancel();
}

bool IVHACD_Compute_f32(
    VHACD::IVHACD* self,
    const float* const points,
    const uint32_t countPoints,
    const uint32_t* const triangles,
    const uint32_t countTriangles,
    const VHACD::IVHACD::Parameters& params) {
    return self->Compute(points, countPoints, triangles, countTriangles, params);
}

bool IVHACD_Compute_f64(
    VHACD::IVHACD* self,
    const double* const points,
    const uint32_t countPoints,
    const uint32_t* const triangles,
    const uint32_t countTriangles,
    const VHACD::IVHACD::Parameters& params) {
    return self->Compute(points, countPoints, triangles, countTriangles, params);
}

uint32_t IVHACD_GetNConvexHulls(const VHACD::IVHACD* self) {
    return self->GetNConvexHulls();
}

void IVHACD_GetConvexHull(
    const VHACD::IVHACD* self,
    const uint32_t index,
    VHACD::IVHACD::ConvexHull& ch) {
    self->GetConvexHull(index, ch);
}

void IVHACD_Clean(VHACD::IVHACD* self) {
    self->Clean();
}

void IVHACD_Release(VHACD::IVHACD* self) {
    self->Release();
}

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
    double centerOfMass[3]) {
    return self->ComputeCenterOfMass(centerOfMass);
}

// In synchronous mode (non-multi-threaded) the state is always 'ready'
// In asynchronous mode, this returns true if the background thread is not still actively computing
// a new solution.  In an asynchronous config the 'IsReady' call will report any update or log
// messages in the caller's current thread.
bool IVHACD_IsReady_typed(const VHACD::IVHACD* self) {
    return self->IsReady();
}

class UserCallbackProxy : public VHACD::IVHACD::IUserCallback {
private:
    void* _self;
    UserCallback _cb;

public:
    UserCallbackProxy(void* self, UserCallback cb) : _self(self), _cb(cb) {
    }

private:
    virtual void Update(
        const double overallProgress,
        const double stageProgress,
        const double operationProgress,
        const char* const stage,
        const char* const operation) override {
        _cb(_self, overallProgress, stageProgress, operationProgress, stage, operation);
    }
};

VHACD::IVHACD::IUserCallback* IVHACD_CreateUserCallback(void* self, UserCallback callback) {
    return new UserCallbackProxy(self, callback);
}

void IVHACD_FreeUserCallback(VHACD::IVHACD::IUserCallback* callback) {
    delete callback;
}

class UserLoggerProxy : public VHACD::IVHACD::IUserLogger {
private:
    UserLogger _cb;

public:
    UserLoggerProxy(UserLogger cb) : _cb(cb) {
    }

private:
    virtual void Log(const char* const msg) override {
        _cb(msg);
    }
};

VHACD::IVHACD::IUserLogger* IVHACD_CreateUserLogger(UserLogger logger) {
    return new UserLoggerProxy(logger);
}

void IVHACD_FreeUserLogger(VHACD::IVHACD::IUserLogger* logger) {
    delete logger;
}