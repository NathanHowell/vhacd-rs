use crate::ParameterError::UnexpectedMode;
use log::debug;
use std::convert::{TryFrom, TryInto};
use std::error::Error;
use std::ffi::CStr;
use std::fmt::{self, Display, Formatter};
use std::ptr;

pub struct VHACD(*mut vhacd_sys::IVHACD);

impl VHACD {
    pub fn new() -> VHACD {
        let ptr = unsafe { vhacd_sys::CreateVHACD() };
        VHACD(ptr)
    }

    pub fn cancel(&mut self) {
        unsafe { vhacd_sys::IVHACD_Cancel(self.0) }
    }

    pub fn is_ready(&self) -> bool {
        unsafe { vhacd_sys::IVHACD_IsReady_typed(self.0) }
    }
}

impl Drop for VHACD {
    fn drop(&mut self) {
        unsafe {
            vhacd_sys::IVHACD_Release(self.0);
        }
    }
}

pub enum Mode {
    Voxel,
    Tetrahedron,
}

pub trait Callback {
    fn update(
        &mut self,
        overall_progress: f64,
        stage_progress: f64,
        operation_progress: f64,
        stage: &str,
        operation: &str,
    );
}

trait Logger {
    fn log(&mut self, message: &str);
}

pub struct Parameters {
    /// maximum concavity
    pub concavity: f64,
    /// controls the bias toward clipping along symmetry planes
    pub alpha: f64,
    /// controls the bias toward clipping along revolution axes
    pub beta: f64,
    /// controls the adaptive sampling of the generated convex-hulls
    pub min_volume_per_convex_hull: f64,
    // IUserCallback* m_callback;
    // IUserLogger* m_logger;
    /// maximum number of voxels generated during the voxelization stage
    pub resolution: u32,
    /// controls the maximum number of triangles per convex-hull
    pub max_num_vertices_per_convex_hull: u16,
    /// controls the granularity of the search for the "best" clipping plane
    pub plane_downsampling: u8,
    /// controls the precision of the convex-hull generation process during the clipping plane selection stage
    pub convex_hull_downsampling: u8,
    /// enable/disable normalizing the mesh before applying the convex decomposition
    pub pca: bool,
    pub mode: Mode,
    pub convex_hull_approximation: bool,
    pub ocl_acceleration: bool,
    /// the maximum number of convex hulls to produce from the merge operation
    pub max_convex_hulls: u32,
    /// project the output convex hull vertices onto the original source mesh to increase the floating point accuracy of the results
    pub project_hull_vertices: bool,
}

impl Default for Parameters {
    fn default() -> Self {
        let params = unsafe {
            let mut params = vhacd_sys::IVHACD_Parameters::new();
            params.Init();
            params
        };

        params.try_into().unwrap()
    }
}

impl From<Parameters> for vhacd_sys::IVHACD_Parameters {
    fn from(value: Parameters) -> Self {
        Self::from(&value)
    }
}

impl From<&Parameters> for vhacd_sys::IVHACD_Parameters {
    fn from(value: &Parameters) -> vhacd_sys::IVHACD_Parameters {
        vhacd_sys::IVHACD_Parameters {
            m_concavity: value.concavity,
            m_alpha: value.alpha,
            m_beta: value.beta,
            m_minVolumePerCH: value.min_volume_per_convex_hull,
            m_callback: ptr::null_mut(),
            m_logger: unsafe { vhacd_sys::IVHACD_CreateUserLogger(Some(logger)) },
            m_resolution: value.resolution,
            m_maxNumVerticesPerCH: value.max_num_vertices_per_convex_hull as u32,
            m_planeDownsampling: value.plane_downsampling as u32,
            m_convexhullDownsampling: value.convex_hull_downsampling as u32,
            m_pca: value.pca as u32,
            m_mode: match value.mode {
                Mode::Voxel => 0,
                Mode::Tetrahedron => 1,
            },
            m_convexhullApproximation: value.convex_hull_approximation as u32,
            m_oclAcceleration: value.ocl_acceleration as u32,
            m_maxConvexHulls: value.max_convex_hulls,
            m_projectHullVertices: value.project_hull_vertices,
        }
    }
}

#[derive(Debug)]
pub enum ParameterError {
    UnexpectedMode(u32),
}

impl Display for ParameterError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for ParameterError {}

impl TryFrom<vhacd_sys::IVHACD_Parameters> for Parameters {
    type Error = ParameterError;

    fn try_from(value: vhacd_sys::IVHACD_Parameters) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

impl TryFrom<&vhacd_sys::IVHACD_Parameters> for Parameters {
    type Error = ParameterError;

    fn try_from(value: &vhacd_sys::IVHACD_Parameters) -> Result<Self, Self::Error> {
        Ok(Parameters {
            concavity: value.m_concavity,
            alpha: value.m_alpha,
            beta: value.m_beta,
            min_volume_per_convex_hull: value.m_minVolumePerCH,
            resolution: value.m_resolution,
            max_num_vertices_per_convex_hull: value.m_maxNumVerticesPerCH as u16,
            plane_downsampling: value.m_planeDownsampling as u8,
            convex_hull_downsampling: value.m_convexhullDownsampling as u8,
            pca: value.m_pca != 0,
            mode: match value.m_mode {
                0 => Mode::Voxel,
                1 => Mode::Tetrahedron,
                _ => return Err(UnexpectedMode(value.m_mode)),
            },
            convex_hull_approximation: value.m_convexhullApproximation != 0,
            ocl_acceleration: value.m_oclAcceleration != 0,
            max_convex_hulls: value.m_maxConvexHulls,
            project_hull_vertices: value.m_projectHullVertices,
        })
    }
}

#[cfg(feature = "ncollide3d")]
pub struct ConvexHullIter<'a, T> {
    vhacd: &'a VHACD,
    next: u32,
    size: u32,
    _witness: std::marker::PhantomData<T>,
}

#[cfg(feature = "ncollide3d")]
impl<'a, T> ConvexHullIter<'a, T> {
    fn new(vhacd: &'a VHACD) -> Self {
        let size = unsafe { vhacd_sys::IVHACD_GetNConvexHulls(vhacd.0) };

        ConvexHullIter {
            vhacd,
            size,
            next: 0,
            _witness: std::marker::PhantomData::default(),
        }
    }
}

trait Points<T> {
    fn points(&self) -> T;
}

trait Indices<T> {
    fn indices(&self) -> T;
}

#[cfg(feature = "ncollide3d")]
impl Points<Vec<ncollide3d::na::Point3<f64>>> for vhacd_sys::IVHACD_ConvexHull {
    fn points(&self) -> Vec<ncollide3d::na::Point3<f64>> {
        let points = self.m_nPoints / 3;
        let mut buf = Vec::with_capacity(points as usize);
        for i in 0..points {
            let off = (i * 3) as isize;
            let point = unsafe {
                ncollide3d::na::Point3::new(
                    *self.m_points.offset(off + 0),
                    *self.m_points.offset(off + 1),
                    *self.m_points.offset(off + 2),
                )
            };
            buf.push(point)
        }
        buf
    }
}

#[cfg(feature = "ncollide3d")]
impl Indices<ncollide3d::procedural::IndexBuffer> for vhacd_sys::IVHACD_ConvexHull {
    fn indices(&self) -> ncollide3d::procedural::IndexBuffer {
        let triangles = self.m_nTriangles / 3;
        let mut buf = Vec::with_capacity(triangles as usize);
        for i in 0..triangles {
            let off = (i * 3) as isize;
            let point = unsafe {
                ncollide3d::na::Point3::new(
                    *self.m_triangles.offset(off + 0),
                    *self.m_triangles.offset(off + 1),
                    *self.m_triangles.offset(off + 2),
                )
            };
            buf.push(point)
        }

        ncollide3d::procedural::IndexBuffer::Unified(buf)
    }
}

extern "C" fn logger(msg: *const ::std::os::raw::c_char) {
    let msg = unsafe { CStr::from_ptr(msg) };
    let msg = msg.to_str().unwrap();
    debug!("{}", msg);
}

#[cfg(feature = "ncollide3d")]
impl Iterator for ConvexHullIter<'_, ncollide3d::procedural::TriMesh<f64>> {
    type Item = ncollide3d::procedural::TriMesh<f64>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next >= self.size {
            None
        } else {
            let mut ch = vhacd_sys::IVHACD_ConvexHull {
                m_points: ptr::null_mut(),
                m_triangles: ptr::null_mut(),
                m_nPoints: 0,
                m_nTriangles: 0,
                m_volume: 0.0,
                m_center: [0.0, 0.0, 0.0],
            };

            unsafe { vhacd_sys::IVHACD_GetConvexHull(self.vhacd.0, self.next, &mut ch) };

            let ch = ch;

            self.next = self.next + 1;

            Some(ncollide3d::procedural::TriMesh::<f64>::new(
                ch.points(),
                None,
                None,
                Some(ch.indices()),
            ))
        }
    }
}

pub trait Compute
where
    Self: Sized,
{
    fn vhacd(&self, params: &Parameters) -> Vec<Self>;
}

#[cfg(feature = "ncollide3d")]
impl Compute for ncollide3d::procedural::TriMesh<f64> {
    fn vhacd(&self, params: &Parameters) -> Vec<Self> {
        let params = params.into();
        let indices = self.flat_indices();
        let points = self
            .coords
            .iter()
            .flat_map(|p| {
                let p = p.coords;
                vec![p.x, p.y, p.z]
            })
            .collect::<Vec<f64>>();

        let vhacd = VHACD::new();
        unsafe {
            vhacd_sys::IVHACD_Compute_f64(
                vhacd.0,
                points.as_ptr(),
                points.len().try_into().unwrap(),
                indices.as_ptr(),
                (indices.len() / 3).try_into().unwrap(),
                &params,
            )
        };

        ConvexHullIter::<ncollide3d::procedural::TriMesh<f64>>::new(&vhacd).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::VHACD;

    #[test]
    fn it_works() {
        let _test = VHACD::new();
    }
}
