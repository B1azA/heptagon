use std::io::{BufReader, Cursor};

use image::GenericImageView;
use anyhow::*;

pub struct Texture {
    texture: wgpu::Texture,
    view: wgpu::TextureView,
    sampler: wgpu::Sampler,
}

impl Texture {
    pub fn texture(&self) -> &wgpu::Texture {
        &self.texture
    }

    pub fn texture_mut(&mut self) -> &mut wgpu::Texture {
        &mut self.texture
    }

    pub fn set_texture(&mut self, texture: wgpu::Texture) {
        self.texture = texture;
    }

    pub fn view(&self) -> &wgpu::TextureView {
        &self.view
    }

    pub fn view_mut(&mut self) -> &mut wgpu::TextureView {
        &mut self.view
    }

    pub fn set_view(&mut self, view: wgpu::TextureView) {
        self.view = view;
    }

    pub fn sampler(&self) -> &wgpu::Sampler {
        &self.sampler
    }

    pub fn sampler_mut(&mut self) -> &mut wgpu::Sampler {
        &mut self.sampler
    }

    pub fn set_sampler(&mut self, sampler: wgpu::Sampler) {
        self.sampler = sampler;
    }

    pub fn new(texture: wgpu::Texture, view: wgpu::TextureView, sampler: wgpu::Sampler
    ) -> Self {
        Self {
            texture,
            view,
            sampler,
        }
    }

    // pub fn from_path(bundle: &super::bundle::Bundle, path: &str, label: &str
    // ) -> Result<Self> {
    //     let bytes = std::fs::read(path).unwrap();
    //     Self::from_bytes(bundle, &bytes, label)
    // }

    pub fn from_bytes(bundle: &super::bundle::Bundle, bytes: &[u8], label: &str
    ) -> Result<Self> {

        let img = image::load_from_memory(bytes).unwrap();
        Self::from_image(bundle, &img, label)
    }

    pub fn empty(bundle: &super::bundle::Bundle,
        dimensions: (u32, u32), label: &str
        ) -> Result<Self> {
            
        let img = image::DynamicImage::new_rgba8(dimensions.0, dimensions.1);
        Self::from_image(bundle, &img, label)
    }

    pub fn from_image(bundle: &super::bundle::Bundle, 
        img: &image::DynamicImage, label: &str) -> Result<Self> {

        let rgba = img.to_rgba8();
        let dimensions = img.dimensions();

        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };

        let texture_format = wgpu::TextureFormat::Rgba8UnormSrgb;

        let texture = bundle.device().create_texture(
            &wgpu::TextureDescriptor {
                label: Some(label),
                size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: texture_format,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST
                    | wgpu::TextureUsages::RENDER_ATTACHMENT,
            }
        );

        bundle.queue().write_texture(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            &rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: std::num::NonZeroU32::new(4 * dimensions.0),
                rows_per_image: std::num::NonZeroU32::new(dimensions.1),
            },
            size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = bundle.device().create_sampler(
            &wgpu::SamplerDescriptor {
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Nearest,
                mipmap_filter: wgpu::FilterMode::Nearest,
                ..Default::default()
            }
        );
        
        Ok(Self { texture, view, sampler })
    }

    pub fn from_r8u_bytes(
        bundle: &super::bundle::Bundle,
        bytes: &[u8], dimensions: (u32, u32),
        label: &str
        ) -> Result<Self> {

        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };

        let texture_format = wgpu::TextureFormat::R8Unorm;

        let texture = bundle.device().create_texture(
            &wgpu::TextureDescriptor {
                label: Some(label),
                size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: texture_format,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            }
        );

        bundle.queue().write_texture(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            &bytes,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: std::num::NonZeroU32::new(dimensions.0),
                rows_per_image: std::num::NonZeroU32::new(dimensions.1),
            },
            size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = bundle.device().create_sampler(
            &wgpu::SamplerDescriptor {
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Nearest,
                mipmap_filter: wgpu::FilterMode::Nearest,
                ..Default::default()
            }
        );
        
        Ok(Self { texture, view, sampler })
    }

    pub fn from_bytes_custom(
        bundle: &super::bundle::Bundle,
        label: &str,
        format: wgpu::TextureFormat, bytes: &[u8], dimensions: (u32, u32), bytes_per_pixel: u8,
        usage: wgpu::TextureUsages) -> Result<Self> {

            let size = wgpu::Extent3d {
                width: dimensions.0,
                height: dimensions.1,
                depth_or_array_layers: 1,
            };
    
            let texture_format = format;
    
            let texture = bundle.device().create_texture(
                &wgpu::TextureDescriptor {
                    label: Some(label),
                    size,
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: wgpu::TextureDimension::D2,
                    format: texture_format,
                    usage,
                }
            );
    
            bundle.queue().write_texture(
                wgpu::ImageCopyTexture {
                    aspect: wgpu::TextureAspect::All,
                    texture: &texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                },
                &bytes,
                wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: std::num::NonZeroU32::new(bytes_per_pixel as u32 * dimensions.0),
                    rows_per_image: std::num::NonZeroU32::new(dimensions.1),
                },
                size,
            );
    
            let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
            let sampler = bundle.device().create_sampler(
                &wgpu::SamplerDescriptor {
                    address_mode_u: wgpu::AddressMode::ClampToEdge,
                    address_mode_v: wgpu::AddressMode::ClampToEdge,
                    address_mode_w: wgpu::AddressMode::ClampToEdge,
                    mag_filter: wgpu::FilterMode::Linear,
                    min_filter: wgpu::FilterMode::Nearest,
                    mipmap_filter: wgpu::FilterMode::Nearest,
                    ..Default::default()
                }
            );
            
            Ok(Self { texture, view, sampler })
    }

    pub fn bind_group_layout(bundle: &super::bundle::Bundle) -> wgpu::BindGroupLayout {
        bundle.device().create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("texture_bindgroup_layout"),
            }
        )
    }

    pub fn bind_group(&self, bundle: &super::bundle::Bundle) -> wgpu::BindGroup {
        bundle.device().create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &Self::bind_group_layout(bundle),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&self.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&self.sampler),
                }
            ],
            label: Some("diffuse_bind_group"),
        })
    }

    pub fn bind_group_with_layout(&self, bundle: &super::bundle::Bundle, layout: &wgpu::BindGroupLayout) -> wgpu::BindGroup {
        bundle.device().create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&self.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&self.sampler),
                }
            ],
            label: Some("diffuse_bind_group"),
        })
    }

    pub fn depth_texture(
        bundle: &super::bundle::Bundle,
        format: wgpu::TextureFormat,
        label: &str
    ) -> Self {    
        let size = wgpu::Extent3d {
            width: bundle.config().width,
            height: bundle.config().height,
            depth_or_array_layers: 1,
        };
        let desc = wgpu::TextureDescriptor {
            label: Some(label),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::TEXTURE_BINDING,
        };
        let texture = bundle.device().create_texture(&desc);

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = bundle.device().create_sampler(
            &wgpu::SamplerDescriptor {
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                mipmap_filter: wgpu::FilterMode::Nearest,
                compare: Some(wgpu::CompareFunction::LessEqual),
                lod_min_clamp: -100.0,
                lod_max_clamp: 100.0,
                ..Default::default()
            }
        );

        Self { texture, view, sampler }
    }
}