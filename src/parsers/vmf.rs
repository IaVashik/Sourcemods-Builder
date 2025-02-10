use log::{debug, trace};
use std::path::Path;

use crate::asset_processor::UniqueAssets;
pub use vmf_forge::{vmf::world::Solid, VmfError, VmfFile, VmfResult};

/// Extracts unique assets from a VMF file.
pub fn get_uniques(path: &Path, uasset: &mut UniqueAssets) -> VmfResult<()> {
    let vmf = VmfFile::open(path)?;

    add_unique_models(&vmf, uasset);
    _process_solids(&vmf.world.solids, uasset);
    Ok(())
}

/// Extracts unique model and material names from VMF entities.
fn add_unique_models(vmf: &VmfFile, uassets: &mut UniqueAssets) {
    debug!("Extracting unique assets from VMF entities...");
    for ent in vmf.entities.iter() {
        if let Some(modelname) = ent.model() {
            if modelname.contains(".vmt") {
                uassets.materials_name
                    .insert(modelname.replace(".vmt", "").into()); // Add material name
                continue;
            }
            uassets.models_name.insert(modelname.into());
        }

        // MATERIALS (if it is a brush entity)
        if let Some(material_name) = ent.get("material") {
            uassets.materials_name.insert(material_name.into());
            trace!("Found material from 'material' key: {}", material_name);
        }
        if let Some(material_name) = ent.get("texture") {
            uassets.materials_name.insert(material_name.into());
            trace!("Found material from 'texture' key: {}", material_name);
        }
        if let Some(solids) = &ent.solids {
            _process_solids(solids, uassets);
        }

        // SOUNDS
        for value in ent.key_values.values() {
            if value.contains(".wav") {
                uassets.sounds_name.insert(value.into());
                trace!("Found sound: {}", value);
            }
        }
    }
}

/// Processes a vector of VMF solids to extract unique material names.
fn _process_solids(solids: &[Solid], uassets: &mut UniqueAssets) {
    for solid in solids {
        for side in &solid.sides {
            let material_path = side.material.to_lowercase();
            if !material_path.contains("tools") {
                trace!("Found material from solid side: {}", material_path);
                uassets.materials_name.insert(material_path.into());
            }
        }
    }
}
