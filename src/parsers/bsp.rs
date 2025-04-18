use crate::asset_processor::UniqueAssets;
use log::trace;
use std::path::Path;
use vbsp::{Bsp, BspResult};
 
/// Extracts unique assets from a BSP map file.
/// Currently, BSP parsing is not implemented.
pub fn get_uniques(path: &Path, uasset: &mut UniqueAssets) -> BspResult<()> {
    trace!("Attempting to read and parse BSP file: {}", path.display()); 
    let data = std::fs::read(path)?;
    let bsp = Bsp::read(&data)?;

    add_unique_models(&bsp, uasset);
    add_texture(&bsp, uasset);
    Ok(())
}

fn is_coord_segment(segment: &str) -> bool {
    !segment.is_empty() && segment.chars().all(|c| c.is_ascii_digit() || c == '-' || c == '_')
}

fn add_texture(bsp: &Bsp, uassets: &mut UniqueAssets) {
    for tex_info_handle in bsp.textures() {
        let name = tex_info_handle.name();
        
        if !name.is_empty() && !name.starts_with("TOOLS/") {
            trace!("Processing texture name: '{}'", name);
            // oh no, this is patch-materials
            if name.starts_with("maps/") {
                let mut name_part = name;
                if let Some(second_slash_pos) = name_part[5..].find('/') {
                    name_part = &name_part[5 + second_slash_pos + 1..];
                }      

                // remove suffix
                if let Some(pos3) = name_part.rfind('_') {
                    if let Some(pos2) = name_part[..pos3].rfind('_') {
                        if let Some(pos1) = name_part[..pos2].rfind('_') {
                            let segment1 = &name_part[pos1 + 1 .. pos2];
                            let segment2 = &name_part[pos2 + 1 .. pos3];
                            let segment3 = &name_part[pos3 + 1 ..];
            
                            if is_coord_segment(segment1) && is_coord_segment(segment2) && is_coord_segment(segment3) {
                                uassets.materials_name.insert(name_part[..pos1].into());
                                continue;
                            }            
                        }
                    }
                }
            }

            uassets.materials_name.insert(name.into());
        }
    }    
}


/// Extracts unique model and material names from BSP entities.
fn add_unique_models(bsp: &Bsp, uassets: &mut UniqueAssets) {
    for fixed_string in &bsp.static_props.dict.name {
        let model_path = fixed_string.as_str();
        uassets.models_name.insert(model_path.into());
    }

    for entity in bsp.entities.iter() {
        for (key, value) in entity.properties() {
            // Process models
            if key == "model" {
                let modelname = value;
                if modelname.ends_with(".vmt") {
                    uassets.materials_name
                        .insert(modelname.replace(".vmt", "").into()); // Add material name
                    continue;
                }
                if modelname.starts_with("models") {
                    uassets.models_name.insert(modelname.into());
                }  
            }

            if key == "texture" || key == "materials" {
                trace!("Found material from '{}' key: {}", key, value);
                uassets.materials_name.insert(value.into());
            } 

            // Process Sounds
            let suffixes = [".wav", ".mp3", ".ogg", ".flac"];
            if suffixes.iter().any(|suffix| {
                value.len() >= suffix.len() &&
                value[value.len() - suffix.len()..].eq_ignore_ascii_case(suffix)
            }) {
                trace!("Found sound: {}", value);
                uassets.sounds_name.insert(value.into());
            }
        }
    }
}