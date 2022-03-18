
pub mod api {
	// Include the output of build.rs
	include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

mod shader;
pub use shader::{Shader, VertexShader, FragmentShader};

mod program;
pub use program::{Program, UniformValue};

mod vertex;
pub use vertex::{Vertex, VertexAttrib, PosVertex, ColorVertex, TexCoordVertex};

mod vbo;
pub use vbo::{VertexBuffer, BufferUsage};

mod vao;
pub use vao::{VertexArray, PrimitiveType};

mod ibo;
pub use ibo::IndexBuffer;

mod tex2d;
pub use tex2d::{Texture2D, TexFilters, TexFilter};


mod event;
pub use event::{Event, EventHandler};

mod window;
pub use window::{Window, WindowBorder, VideoMode, VSync};

mod perf;
pub use perf::Perf;

mod keys;
pub use keys::{Key, Modifiers};
