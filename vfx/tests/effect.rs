// Standard Uses

// Crate Uses

// External Uses
// use lyketo_vfx::parsers::effect;
// use lyketo_vfx::parsers::effect::version2::legacy::EffectV1Legacy;


const EFFECTS_DIRECTORY: &str = "../../Bevy/test_data/effects/";

#[test]
fn parse_effect_version1_file() {
    let _effect_path = format!("{}/battley_03_full.mde", EFFECTS_DIRECTORY);

    // let _effect = effect::version1::legacy::parse_file(&effect_path);
}

#[test]
fn parse_effect_version2_file() {
    let _effect_path = format!("{}/flag_yellow.mde", EFFECTS_DIRECTORY);
    // let effect: EffectV1Legacy  =  serde_json::parse_file(&effect_path);
}

