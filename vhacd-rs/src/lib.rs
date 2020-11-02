use std::marker::PhantomData;
use std::ptr;

struct VHACD(*mut vhacd_sys::IVHACD);

impl VHACD {
    fn new() -> VHACD {
        let ptr = unsafe { vhacd_sys::CreateVHACD() };
        VHACD(ptr)
    }

    fn cancel(&mut self) {
        unsafe { vhacd_sys::IVHACD_Cancel(self.0) }
    }

    fn is_ready(&self) -> bool {
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

enum Mode {
    Voxel,
    Tetrahedron,
}

struct Parameters {
    concavity: f64,
    alpha: f64,
    beta: f64,
    min_volume_per_convex_hull: f64,
    // IUserCallback* m_callback;
    // IUserLogger* m_logger;
    resolution: u32,
    max_num_vertices_per_ch: u16,
    plane_downsampling: u8,
    convex_hull_downsampling: u8,
    pca: bool,
    mode: Mode,
    convex_hull_approximation: bool,
    ocl_acceleration: bool,
    max_convex_hulls: u32,
    project_hull_vertices: bool,
}

impl Default for Parameters {
    fn default() -> Self {
        let params = unsafe {
            let mut params = vhacd_sys::IVHACD_Parameters::new();
            params.Init();
            params
        };

        params.into()
    }
}

impl Into<vhacd_sys::IVHACD_Parameters> for &Parameters {
    fn into(self) -> vhacd_sys::IVHACD_Parameters {
        vhacd_sys::IVHACD_Parameters {
            m_concavity: self.concavity,
            m_alpha: self.alpha,
            m_beta: self.beta,
            m_minVolumePerCH: self.min_volume_per_convex_hull,
            m_callback: ptr::null_mut(),
            m_logger: ptr::null_mut(),
            m_resolution: self.resolution,
            m_maxNumVerticesPerCH: self.max_num_vertices_per_ch as u32,
            m_planeDownsampling: self.plane_downsampling as u32,
            m_convexhullDownsampling: self.convex_hull_downsampling as u32,
            m_pca: self.pca as u32,
            m_mode: match self.mode {
                Mode::Voxel => 0,
                Mode::Tetrahedron => 1,
            },
            m_convexhullApproximation: self.convex_hull_approximation as u32,
            m_oclAcceleration: self.ocl_acceleration as u32,
            m_maxConvexHulls: self.max_convex_hulls,
            m_projectHullVertices: self.project_hull_vertices,
        }
    }
}

impl Into<Parameters> for vhacd_sys::IVHACD_Parameters {
    fn into(self) -> Parameters {
        Parameters {
            concavity: self.m_concavity,
            alpha: self.m_alpha,
            beta: self.m_beta,
            min_volume_per_convex_hull: self.m_minVolumePerCH,
            resolution: self.m_resolution,
            max_num_vertices_per_ch: self.m_maxNumVerticesPerCH as u16,
            plane_downsampling: self.m_planeDownsampling as u8,
            convex_hull_downsampling: self.m_convexhullDownsampling as u8,
            pca: self.m_pca != 0,
            mode: match self.m_mode {
                0 => Mode::Voxel,
                1 => Mode::Tetrahedron,
                _ => panic!("Unexpected mode {}", self.m_mode),
            },
            convex_hull_approximation: self.m_convexhullApproximation != 0,
            ocl_acceleration: self.m_oclAcceleration != 0,
            max_convex_hulls: self.m_maxConvexHulls,
            project_hull_vertices: self.m_projectHullVertices,
        }
    }
}

struct ConvexHullIter<'a, T> {
    vhacd: &'a mut VHACD,
    next: u32,
    size: u32,
    _witness: PhantomData<T>,
}

impl<T> Iterator for ConvexHullIter<'_, T> {
    type Item = Vec<[f64; 3]>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next >= self.size {
            None
        } else {
            let ch = {
                let mut ch = vhacd_sys::IVHACD_ConvexHull {
                    m_points: ptr::null_mut(),
                    m_triangles: ptr::null_mut(),
                    m_nPoints: 0,
                    m_nTriangles: 0,
                    m_volume: 0.0,
                    m_center: [0.0, 0.0, 0.0],
                };

                unsafe { vhacd_sys::IVHACD_GetConvexHull(self.vhacd.0, self.next, &mut ch) };

                ch
            };

            let triangles = ch.m_nTriangles / 3;
            let mut buf = Vec::with_capacity(triangles as usize);
            for i in 0..triangles {
                let off = (i * 3) as isize;
                let tri = unsafe {
                    [
                        *ch.m_points.offset(*ch.m_triangles.offset(off + 0) as isize),
                        *ch.m_points.offset(*ch.m_triangles.offset(off + 1) as isize),
                        *ch.m_points.offset(*ch.m_triangles.offset(off + 2) as isize),
                    ]
                };
                buf.push(tri)
            }

            Some(buf)
        }
    }
}

trait Compute<T> {
    // type It

    // fn compute() -> dyn IntoIterator<Item = R>;
}
// struct Parameters(IVHACD_Parameters);

#[cfg(test)]
mod tests {
    use crate::VHACD;

    #[test]
    fn it_works() {
        let _test = VHACD::new();
    }
}
