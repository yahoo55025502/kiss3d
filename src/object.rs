use std::sys;
use std::libc;
use std::num::One;
use std::ptr;
use glcore::types::GL_VERSION_1_0::*;
use glcore::functions::GL_VERSION_1_1::*;
use glcore::functions::GL_VERSION_2_0::*;
use glcore::consts::GL_VERSION_1_1::*;
use nalgebra::traits::homogeneous::ToHomogeneous;
use nalgebra::traits::indexable::Indexable;
use nalgebra::adaptors::transform::Transform;
use nalgebra::adaptors::rotmat::Rotmat;
use nalgebra::mat::{Mat3, Mat4};
use nalgebra::vec::Vec3;

type Transform3d = Transform<Rotmat<Mat3<f64>>, Vec3<f64>>;
type Scale3d     = Mat3<GLfloat>;

pub struct GeometryIndices
{
  priv offset: uint,
  priv size:   i32
}

impl GeometryIndices
{
  pub fn new(offset: uint, size: i32) -> GeometryIndices
  {
    GeometryIndices {
      offset: offset,
      size:   size
    }
  }
}

pub struct Object
{
  priv scale:     Scale3d,
  priv transform: Transform3d,
  priv color:     Vec3<f32>,
  priv geometry:  GeometryIndices
}

impl Object
{
  pub fn new(geometry: GeometryIndices,
             r: f32,
             g: f32,
             b: f32,
             sx: GLfloat,
             sy: GLfloat,
             sz: GLfloat) -> Object
  {
    Object {
      scale:     Mat3::new( [
                              sx, 0.0, 0.0,
                              0.0, sy, 0.0,
                              0.0, 0.0, sz,
                            ] ),
      transform: One::one(),
      geometry:  geometry,
      color:     Vec3::new([r, g, b])
    }
  }

  pub fn upload(&self,
                color_location:            i32,
                transform_location:        i32,
                scale_location:            i32,
                normal_transform_location: i32)
  {
    let formated_transform:  Mat4<f64> = self.transform.to_homogeneous();
    let formated_ntransform: Mat3<f64> = self.transform.submat().submat();

    // we convert the matrix elements and do the transposition at the same time
    let transform_glf = Mat4::new ([
      formated_transform.at((0, 0)) as GLfloat,
      formated_transform.at((1, 0)) as GLfloat,
      formated_transform.at((2, 0)) as GLfloat,
      formated_transform.at((3, 0)) as GLfloat,

      formated_transform.at((0, 1)) as GLfloat,
      formated_transform.at((1, 1)) as GLfloat,
      formated_transform.at((2, 1)) as GLfloat,
      formated_transform.at((3, 1)) as GLfloat,

      formated_transform.at((0, 2)) as GLfloat,
      formated_transform.at((1, 2)) as GLfloat,
      formated_transform.at((2, 2)) as GLfloat,
      formated_transform.at((3, 2)) as GLfloat,

      formated_transform.at((0, 3)) as GLfloat,
      formated_transform.at((1, 3)) as GLfloat,
      formated_transform.at((2, 3)) as GLfloat,
      formated_transform.at((3, 3)) as GLfloat,
    ]);

    let ntransform_glf = Mat3::new ([
      formated_ntransform.at((0, 0)) as GLfloat,
      formated_ntransform.at((1, 0)) as GLfloat,
      formated_ntransform.at((2, 0)) as GLfloat,
      formated_ntransform.at((0, 1)) as GLfloat,
      formated_ntransform.at((1, 1)) as GLfloat,
      formated_ntransform.at((2, 1)) as GLfloat,
      formated_ntransform.at((0, 2)) as GLfloat,
      formated_ntransform.at((1, 2)) as GLfloat,
      formated_ntransform.at((2, 2)) as GLfloat,
    ]);

    unsafe {
      glUniformMatrix4fv(transform_location,
                         1,
                         GL_FALSE,
                         ptr::to_unsafe_ptr(&transform_glf.mij[0]));

      glUniformMatrix3fv(normal_transform_location,
                         1,
                         GL_FALSE,
                         ptr::to_unsafe_ptr(&ntransform_glf.mij[0]));

      glUniformMatrix3fv(scale_location,
                         1,
                         GL_FALSE,
                         ptr::to_unsafe_ptr(&self.scale.mij[0]));

      glUniform3f(color_location, self.color.at[0], self.color.at[1], self.color.at[2]);
      glDrawElements(GL_TRIANGLES,
                     self.geometry.size,
                     GL_UNSIGNED_INT,
                     self.geometry.offset * sys::size_of::<GLuint>() as *libc::c_void);
    }
  }

  pub fn transformation<'r>(&'r mut self) -> &'r mut Transform3d
  { &mut self.transform }

  pub fn set_color(@mut self, r: f32, g: f32, b: f32) -> @mut Object
  {
    self.color.at[0] = r;
    self.color.at[1] = g;
    self.color.at[2] = b;

    self
  }
}