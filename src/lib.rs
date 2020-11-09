use std::os::raw::{c_char, c_uchar, c_uint, c_void};
use std::ptr;
use std::slice;
use std::str;

#[macro_use]
extern crate bitflags;

#[repr(C)]
pub struct AiScene {
    pub flags: c_uint,                   // unsigned int mFlags;
    pub root_node: *mut c_void,          //C_STRUCT aiNode* mRootNode;
    pub num_meshes: c_uint,              //unsigned int mNumMeshes;
    pub meshes: *mut *mut AiMesh,        //C_STRUCT aiMesh** mMeshes;
    pub num_materials: c_uint,           //unsigned int mNumMaterials;
    pub materials: *mut *mut AiMaterial, //C_STRUCT aiMaterial** mMaterials;
    pub num_animations: c_uint,          //unsigned int mNumAnimations;
    pub animations: *mut *mut c_void,    //C_STRUCT aiAnimation** mAnimations;
    pub num_textures: c_uint,            //unsigned int mNumTextures;
    pub textures: *mut *mut c_void,      //C_STRUCT aiTexture** mTextures;
    pub num_lights: c_uint,              //unsigned int mNumLights;
    pub lights: *mut *mut c_void,        //C_STRUCT aiLight** mLights;
    pub num_cameras: c_uint,             // unsigned int mNumCameras;
    pub cameras: *mut *mut c_void,       //C_STRUCT aiCamera** mCameras;
    pub metadata: *mut c_void,           //C_STRUCT aiMetadata* mMetaData;
    private: *mut c_void,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AiVector3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AiAABB {
    pub min: AiVector3D,
    pub max: AiVector3D,
}

#[repr(C)]
#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub struct AiColor4D {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

pub const MAXLEN: usize = 1024;

#[repr(C)]
#[derive(Copy)]
pub struct AiString {
    pub length: u32,
    pub data: [c_uchar; MAXLEN],
}

impl Default for AiString {
    fn default() -> AiString {
        AiString {
            length: 0,
            data: [0; MAXLEN],
        }
    }
}

impl Clone for AiString {
    fn clone(&self) -> AiString {
        *self
    }
}

impl AsRef<str> for AiString {
    fn as_ref(&self) -> &str {
        str::from_utf8(&self.data[0..self.length as usize]).unwrap()
    }
}

#[repr(C)]
pub struct AiFace {
    pub num_indices: c_uint,
    pub indices: *mut c_uint,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AiPropertyTypeInfo {
    Float = 0x1,
    Double = 0x2,
    String = 0x3,
    Integer = 0x4,
    Buffer = 0x5,
}

#[repr(C)]
pub struct AiMaterialProperty {
    pub key: AiString,                     // C_STRUCT aiString mKey;
    pub semantic: c_uint,                  // unsigned int mSemantic;
    pub index: c_uint,                     // unsigned int mIndex;
    pub data_length: c_uint,               // unsigned int mDataLength;
    pub property_type: AiPropertyTypeInfo, // C_ENUM aiPropertyTypeInfo mType;
    pub data: *mut c_char,                 // char* mData;
}

#[repr(C)]
pub struct AiMaterial {
    pub properties: *mut *mut AiMaterialProperty,
    pub num_properties: c_uint,
    pub num_allocated: c_uint,
}

pub const AI_MAX_NUMBER_OF_COLOR_SETS: usize = 0x8;
pub const AI_MAX_NUMBER_OF_TEXTURECOORDS: usize = 0x8;

#[repr(C)]
pub struct AiMesh {
    pub primitive_types: c_uint, // unsigned int mPrimitiveTypes; aiPrimitiveType
    pub num_vertices: c_uint,    // unsigned int mNumVertices;
    pub num_faces: c_uint,       //unsigned int mNumFaces;
    pub vertices: *mut AiVector3D, // C_STRUCT aiVector3D* mVertices;
    pub normals: *mut AiVector3D, // C_STRUCT aiVector3D* mNormals;
    pub tangents: *mut AiVector3D, // C_STRUCT aiVector3D* mTangents;
    pub bitangents: *mut AiVector3D, // C_STRUCT aiVector3D* mBitangents;
    pub colors: [*mut AiColor4D; AI_MAX_NUMBER_OF_COLOR_SETS], // C_STRUCT aiColor4D* mColors[AI_MAX_NUMBER_OF_COLOR_SETS];
    pub texture_coords: [*mut AiVector3D; AI_MAX_NUMBER_OF_TEXTURECOORDS], // C_STRUCT aiVector3D* mTextureCoords[AI_MAX_NUMBER_OF_TEXTURECOORDS];
    pub num_uv_components: [c_uint; AI_MAX_NUMBER_OF_TEXTURECOORDS], // unsigned int mNumUVComponents[AI_MAX_NUMBER_OF_TEXTURECOORDS];
    pub faces: *mut AiFace,                                          // C_STRUCT aiFace* mFaces;
    pub num_bones: c_uint,                                           // unsigned int mNumBones;
    pub bones: *mut c_void,                                          // C_STRUCT aiBone** mBones;
    pub material_index: c_uint,                                      // unsigned int mMaterialIndex;
    pub name: AiString,                                              // C_STRUCT aiString mName;
    pub num_anim_meshes: c_uint,                                     // unsigned int mNumAnimMeshes;
    pub anim_meshes: *mut *mut c_void, // C_STRUCT aiAnimMesh** mAnimMeshes;
    pub method: c_uint,                // unsigned int mMethod;
    pub aabb: AiAABB,                  // C_STRUCT aiAABB mAABB;
}

impl AiMesh {
    pub fn name(&self) -> String {
        unsafe {
            CStr::from_ptr(self.name.data.as_ptr() as _)
                .to_string_lossy()
                .into_owned()
        }
    }

    pub fn verts(&self) -> &[AiVector3D] {
        assert!(!self.vertices.is_null());
        unsafe { slice::from_raw_parts(self.vertices, self.num_vertices as _) }
    }

    pub fn normals(&self) -> &[AiVector3D] {
        assert!(!self.normals.is_null());
        unsafe { slice::from_raw_parts(self.normals, self.num_vertices as _) }
    }

    pub fn tangents(&self) -> &[AiVector3D] {
        assert!(!self.tangents.is_null());
        unsafe { slice::from_raw_parts(self.tangents, self.num_vertices as _) }
    }

    pub fn binormals(&self) -> &[AiVector3D] {
        assert!(!self.bitangents.is_null());
        unsafe { slice::from_raw_parts(self.bitangents, self.num_vertices as _) }
    }

    pub fn texcoords(&self, index: usize) -> Option<&[AiVector3D]> {
        if index < AI_MAX_NUMBER_OF_TEXTURECOORDS && !self.texture_coords[index].is_null() {
            Some(unsafe {
                slice::from_raw_parts(self.texture_coords[index], self.num_vertices as _)
            })
        } else {
            None
        }
    }

    pub fn triangles(&self) -> impl Iterator<Item = (u32, u32, u32)> {
        let faces = unsafe { slice::from_raw_parts(self.faces, self.num_faces as _) };

        faces.iter().map(|f| {
            assert_eq!(f.num_indices, 3);
            let face_indices = unsafe { slice::from_raw_parts(f.indices, f.num_indices as _) };

            (face_indices[0], face_indices[1], face_indices[2])
        })
    }
}

impl AiMaterial {
    pub fn name(&self) -> Option<String> {
        let mut name = AiString::default();
        let result =
            unsafe { aiGetMaterialString(self, "?mat.name\0".as_ptr() as _, 0, 0, &mut name as _) };

        match result {
            AiReturn::Success => unsafe {
                Some(
                    CStr::from_ptr(name.data.as_ptr() as _)
                        .to_string_lossy()
                        .into_owned(),
                )
            },
            _ => None,
        }
    }

    fn color(&self, name: &'static str) -> Option<AiColor4D> {
        let mut color = AiColor4D::default();
        let result = unsafe { aiGetMaterialColor(self, name.as_ptr() as _, 0, 0, &mut color) };

        match result {
            AiReturn::Success => Some(color),
            _ => None,
        }
    }

    pub fn diffuse_color(&self) -> Option<AiColor4D> {
        self.color("$clr.diffuse\0")
    }

    pub fn emissive_color(&self) -> Option<AiColor4D> {
        self.color("$clr.emissive\0")
    }
}

bitflags! {
    #[repr(C)]
    pub struct AiPrimitiveType: c_uint {
        const AI_PRIMITIVE_TYPE_POINT       = 0x1;
        const AI_PRIMITIVE_TYPE_LINE        = 0x2;
        const AI_PRIMITIVE_TYPE_TRIANGLE    = 0x4;
        const AI_PRIMITIVE_TYPE_POLYGON     = 0x8;
    }
}

bitflags! {
    #[repr(C)]
    pub struct AiPostProcessSteps: c_uint {
        const AI_PROCESS_CALC_TANGENT_SPACE = 0x1;
        const AI_PROCESS_JOIN_IDENTICAL_VERTICES = 0x2;
        const AI_PROCESS_MAKE_LEFT_HANDED = 0x4;
        const AI_PROCESS_TRIANGULATE = 0x8;
        const AI_PROCESS_REMOVE_COMPONENT = 0x10;
        const AI_PROCESS_GEN_NORMALS = 0x20;
        const AI_PROCESS_GEN_SMOOTH_NORMALS = 0x40;
        const AI_PROCESS_SPLIT_LARGE_MESHES = 0x80;
        const AI_PROCESS_PRE_TRANSFORM_VERTICES = 0x100;
        const AI_PROCESS_LIMIT_BONE_WEIGHTS = 0x200;
        const AI_PROCESS_VALIDATE_DATA_STRUCTURE = 0x400;
        const AI_PROCESS_IMPROVE_CACHE_LOCALITY = 0x800;
        const AI_PROCESS_REMOVE_REDUNDANT_MATERIALS = 0x1000;
        const AI_PROCESS_FIX_INFACING_NORMALS = 0x2000;
        const AI_PROCESS_POPULATE_ARMATURE_DATA = 0x4000;
        const AI_PROCESS_SORT_BY_PTYPE = 0x8000;
        const AI_PROCESS_FIND_DEGENERATES = 0x10000;
        const AI_PROCESS_FIND_INVALID_DATA = 0x20000;
        const AI_PROCESS_GEN_UVCOORDS = 0x40000;
        const AI_PROCESS_TRANSFORM_UVCOORDS = 0x80000;
        const AI_PROCESS_FIND_INSTANCES = 0x100000;
        const AI_PROCESS_OPTIMIZE_MESHES  = 0x200000;
        const AI_PROCESS_OPTIMIZE_GRAPH  = 0x400000;
        const AI_PROCESS_FLIP_UVS = 0x800000;
        const AI_PROCESS_FLIP_WINDING_ORDER  = 0x1000000;
        const AI_PROCESS_SPLIT_BY_BONE_COUNT  = 0x2000000;
        const AI_PROCESS_DEBONE  = 0x4000000;
        const AI_PROCESS_GLOBAL_SCALE = 0x8000000;
        const AI_PROCESS_EMBED_TEXTURES  = 0x10000000;
        const AI_PROCESS_FORCE_GEN_NORMALS = 0x20000000;
        const AI_PROCESS_DROP_NORMALS = 0x40000000;
        const AI_PROCESS_GEN_BOUNDING_BOXES = 0x80000000;


        const AI_PROCESS_PRESET_TARGET_REALTIME_QUALITY =
            Self::AI_PROCESS_CALC_TANGENT_SPACE.bits              |
            Self::AI_PROCESS_GEN_SMOOTH_NORMALS.bits              |
            Self::AI_PROCESS_JOIN_IDENTICAL_VERTICES.bits         |
            Self::AI_PROCESS_IMPROVE_CACHE_LOCALITY.bits          |
            Self::AI_PROCESS_LIMIT_BONE_WEIGHTS.bits              |
            Self::AI_PROCESS_REMOVE_REDUNDANT_MATERIALS.bits      |
            Self::AI_PROCESS_SPLIT_LARGE_MESHES.bits              |
            Self::AI_PROCESS_TRIANGULATE.bits                     |
            Self::AI_PROCESS_GEN_UVCOORDS.bits                    |
            Self::AI_PROCESS_SORT_BY_PTYPE.bits                   |
            Self::AI_PROCESS_FIND_DEGENERATES.bits                |
            Self::AI_PROCESS_FIND_INVALID_DATA.bits;

        const AI_PROCESS_PRESET_TARGET_REALTIME_MAX_QUALITY =
            Self::AI_PROCESS_PRESET_TARGET_REALTIME_QUALITY.bits  |
            Self::AI_PROCESS_FIND_INSTANCES.bits                  |
            Self::AI_PROCESS_VALIDATE_DATA_STRUCTURE.bits         |
            Self::AI_PROCESS_OPTIMIZE_MESHES.bits;
    }
}

pub type AiLogStreamCallback = Option<unsafe extern "system" fn(*const c_char, *mut c_char)>;

#[repr(C)]
pub struct AiLogStream {
    pub callback: AiLogStreamCallback,
    pub user: *mut c_void,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AiReturn {
    Success = 0,
    Failure = -1,
    OutOfMemory = -3,
}

extern "C" {
    pub fn aiImportFile(file: *const c_char, flags: c_uint) -> *const AiScene;

    pub fn aiAttachLogStream(stream: *const AiLogStream);

    pub fn aiDetachLogStream(stream: *const AiLogStream) -> AiReturn;

    pub fn aiDetachAllLogStreams();

    pub fn aiReleaseImport(scene: *const AiScene);

    pub fn aiGetMaterialString(
        mat: *const AiMaterial,
        key: *const c_char,
        property_type: c_uint,
        index: c_uint,
        output: *mut AiString,
    ) -> AiReturn;

    pub fn aiGetMaterialColor(
        mat: *const AiMaterial, //const aiMaterial *pMat,
        key: *const c_char,     //const char *pKey,
        property_type: c_uint,  //unsigned int type,
        index: c_uint,          // unsigned int index,
        output: *mut AiColor4D, //aiColor4D *pOut
    ) -> AiReturn;
}

use std::ffi::CStr;

unsafe extern "system" fn log(msg: *const c_char, _user: *mut c_char) {
    println!("assimp: {:?}", CStr::from_ptr(msg));
}

// simple wrapper
pub struct Scene<'a> {
    scene: &'a AiScene,
}

impl<'a> Scene<'a> {
    pub fn new(path: &str) -> Option<Scene<'a>> {
        let maybe_scene = unsafe {
            let log_stream = AiLogStream {
                callback: Some(log),
                user: ptr::null_mut(),
            };
            aiAttachLogStream(&log_stream);

            let flags = AiPostProcessSteps::AI_PROCESS_PRESET_TARGET_REALTIME_MAX_QUALITY.bits
                | AiPostProcessSteps::AI_PROCESS_PRE_TRANSFORM_VERTICES.bits;
            let scene_ptr = aiImportFile(path.as_ptr() as _, flags);
            aiDetachAllLogStreams();

            scene_ptr.as_ref()
        };

        if let Some(scene) = maybe_scene {
            Some(Scene { scene })
        } else {
            None
        }
    }

    pub fn num_meshes(&self) -> usize {
        self.scene.num_meshes as _
    }

    pub fn meshes(&self) -> impl Iterator<Item = &'a AiMesh> + 'a {
        let meshes =
            unsafe { slice::from_raw_parts(self.scene.meshes, self.scene.num_meshes as _) };
        meshes.iter().map(|p| unsafe { p.as_ref().unwrap() })
    }

    pub fn num_materials(&self) -> usize {
        self.scene.num_materials as _
    }

    pub fn materials(&self) -> impl Iterator<Item = &'a AiMaterial> + 'a {
        let materials =
            unsafe { slice::from_raw_parts(self.scene.materials, self.scene.num_materials as _) };
        materials.iter().map(|p| unsafe { p.as_ref().unwrap() })
    }

    pub fn material_names(&self) -> impl Iterator<Item = String> + 'a {
        let materials =
            unsafe { slice::from_raw_parts(self.scene.materials, self.scene.num_materials as _) };
        materials.iter().map(|p| {
            let material = unsafe { p.as_ref().unwrap() };
            let mut name = AiString::default();
            unsafe {
                aiGetMaterialString(material, "?mat.name\0".as_ptr() as _, 0, 0, &mut name as _);
            }

            unsafe {
                CStr::from_ptr(name.data.as_ptr() as _)
                    .to_string_lossy()
                    .into_owned()
            }
        })
    }
}
