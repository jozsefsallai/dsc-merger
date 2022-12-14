use crate::{
    common::Game,
    error::{ApplicationError, ApplicationResult},
};

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Opcode {
    AGEAGE_CTRL,
    AIM,
    ANNOTATION,
    AOTO_CAP,
    AUTO_BLINK,
    AUTO_CAPTURE_BEGIN,
    BANK_BRANCH,
    BANK_END,
    BAR_POINT,
    BAR_TIME_SET,
    BEAT_POINT,
    BLOOM,
    CHANGE_FIELD,
    CHARA_ALPHA,
    CHARA_COLOR,
    CHARA_EFFECT,
    CHARA_EFFECT_CHARA_LIGHT,
    CHARA_HEIGHT_ADJUST,
    CHARA_LIGHT,
    CHARA_POS_ADJUST,
    CHARA_SHADOW_QUALITY,
    CHARA_SIZE,
    CHROMATIC_ABERRATION,
    CLOTH_WET,
    COLOR_COLLE,
    COLOR_CORRECTION,
    COMMON_EFFECT_AET_FRONT,
    COMMON_EFFECT_AET_FRONT_LOW,
    COMMON_EFFECT_PARTICLE,
    COMMON_LIGHT,
    CREDIT_TITLE,
    CROSSFADE,
    DATA_CAMERA,
    DATA_CAMERA_START,
    DOF,
    DUMMY,
    EDIT_BLUSH,
    EDIT_CAMERA,
    EDIT_CAMERA_BOX,
    EDIT_CHANGE_FIELD,
    EDIT_DISP,
    EDIT_EFFECT,
    EDIT_EXPRESSION,
    EDIT_EYE,
    EDIT_EYELID,
    EDIT_EYELID_ANIM,
    EDIT_EYE_ANIM,
    EDIT_FACE,
    EDIT_HAND_ANIM,
    EDIT_INSTRUMENT_ITEM,
    EDIT_ITEM,
    EDIT_LYRIC,
    EDIT_MODE_SELECT,
    EDIT_MOTION,
    EDIT_MOTION_F,
    EDIT_MOTION_LOOP,
    EDIT_MOT_SMOOTH_LEN,
    EDIT_MOUTH,
    EDIT_MOUTH_ANIM,
    EDIT_MOVE,
    EDIT_MOVE_XYZ,
    EDIT_SHADOW,
    EDIT_STAGE_PARAM,
    EDIT_TARGET,
    EFFECT,
    EFFECT_OFF,
    ENABLE_COMMON_LIGHT_TO_CHARA,
    ENABLE_FXAA,
    ENABLE_REFLECTION,
    ENABLE_TEMPORAL_AA,
    END,
    EVENT_JUDGE,
    EXPRESSION,
    EYE_ANIM,
    FACE_TYPE,
    FADE,
    FADEIN_FIELD,
    FADEOUT_FIELD,
    FADE_MODE,
    FOG,
    FOG_ENABLE,
    GAZE,
    HAND_ANIM,
    HAND_ITEM,
    HAND_SCALE,
    HIDE_FIELD,
    IBL_COLOR,
    ITEM_ALPHA,
    ITEM_ANIM,
    ITEM_ANIM_ATTACH,
    ITEM_LIGHT,
    LIGHT_AUTH,
    LIGHT_POS,
    LIGHT_ROT,
    LOOK_ANIM,
    LOOK_CAMERA,
    LOOK_CAMERA_FACE_LIMIT,
    LYRIC,
    LYRIC_2,
    LYRIC_READ,
    LYRIC_READ_2,
    MANUAL_CAPTURE,
    MAN_CAP,
    MARKER,
    MIKUDAYO_ADJUST,
    MIKU_DISP,
    MIKU_MOVE,
    MIKU_ROT,
    MIKU_SHADOW,
    MODE_SELECT,
    MOUTH_ANIM,
    MOVE_CAMERA,
    MOVE_FIELD,
    MOVIE_CUT,
    MOVIE_CUT_CHG,
    MOVIE_DISP,
    MOVIE_PLAY,
    MUSIC_PLAY,
    NEAR_CLIP,
    OSAGE_MV_CCL,
    OSAGE_STEP,
    PARTS_DISP,
    PSE,
    PV_AUTH_LIGHT_PRIORITY,
    PV_BRANCH_MODE,
    PV_CHARA_LIGHT,
    PV_END,
    PV_END_FADEOUT,
    PV_STAGE_LIGHT,
    REFLECTION,
    REFLECTION_QUALITY,
    RESERVE,
    SATURATE,
    SCENE_FADE,
    SCENE_ROT,
    SET_CAMERA,
    SET_CHARA,
    SET_MOTION,
    SET_PLAYDATA,
    SET_STAGE_EFFECT_ENV,
    SE_EFFECT,
    SHADOWHEIGHT,
    SHADOWPOS,
    SHADOW_CAST,
    SHADOW_RANGE,
    SHIMMER,
    SONG_EFFECT,
    SONG_EFFECT_ALPHA_SORT,
    SONG_EFFECT_ATTACH,
    STAGE_EFFECT,
    STAGE_LIGHT,
    STAGE_SHADOW,
    STAGE_SHADOW_QUALITY,
    SUBFRAMERENDER,
    TARGET,
    TARGET_EFFECT,
    TARGET_FLAG,
    TARGET_FLYING_TIME,
    TECH_DEMO_GESUTRE,
    TIME,
    TONE_MAP,
    TONE_TRANS,
    TOON,
    TOON_EDGE,
    TOON???EDGE,
    VR_CHARA_PSMOVE,
    VR_CHEER,
    VR_CHEMICAL_LIGHT_COLOR,
    VR_LIVE_CHARA_VOICE,
    VR_LIVE_CHEER,
    VR_LIVE_CLONE,
    VR_LIVE_FLY,
    VR_LIVE_GESTURE,
    VR_LIVE_HAIR_OSAGE,
    VR_LIVE_LOOK_CAMERA,
    VR_LIVE_MOB,
    VR_LIVE_MOVIE,
    VR_LIVE_ONESHOT_EFFECT,
    VR_LIVE_PRESENT,
    VR_LIVE_TRANSFORM,
    VR_LOOP_EFFECT,
    VR_MOVE_PATH,
    VR_SET_BASE,
    VR_TECH_DEMO_EFFECT,
    VR_TRANSFORM,
    WIND,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OpcodeMeta {
    pub id: i32,
    pub opcode: Opcode,
    pub param_count: usize,
}

impl OpcodeMeta {
    pub fn new(id: i32, opcode: Opcode, param_count: usize) -> Self {
        Self {
            id,
            opcode,
            param_count,
        }
    }
}

#[derive(Debug, Clone, Eq)]
pub struct Command {
    pub meta: OpcodeMeta,
    pub args: Vec<i32>,
}

impl Command {
    pub fn new(meta: OpcodeMeta, args: Vec<i32>) -> Self {
        Self { meta, args }
    }

    pub fn to_string(&self) -> String {
        let command_name = format!("{:?}", self.meta.opcode);
        let args = self
            .args
            .iter()
            .map(|arg| arg.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        format!("{}({});", command_name, args)
    }

    pub fn get_opcode_meta(game: Game, raw: i32) -> ApplicationResult<OpcodeMeta> {
        match game {
            Game::F => match raw {
                0 => Ok(OpcodeMeta::new(raw, Opcode::END, 0)),
                1 => Ok(OpcodeMeta::new(raw, Opcode::TIME, 1)),
                2 => Ok(OpcodeMeta::new(raw, Opcode::MIKU_MOVE, 4)),
                3 => Ok(OpcodeMeta::new(raw, Opcode::MIKU_ROT, 2)),
                4 => Ok(OpcodeMeta::new(raw, Opcode::MIKU_DISP, 2)),
                5 => Ok(OpcodeMeta::new(raw, Opcode::MIKU_SHADOW, 2)),
                6 => Ok(OpcodeMeta::new(raw, Opcode::TARGET, 11)),
                7 => Ok(OpcodeMeta::new(raw, Opcode::SET_MOTION, 4)),
                8 => Ok(OpcodeMeta::new(raw, Opcode::SET_PLAYDATA, 2)),
                9 => Ok(OpcodeMeta::new(raw, Opcode::EFFECT, 6)),
                10 => Ok(OpcodeMeta::new(raw, Opcode::FADEIN_FIELD, 2)),
                11 => Ok(OpcodeMeta::new(raw, Opcode::EFFECT_OFF, 1)),
                12 => Ok(OpcodeMeta::new(raw, Opcode::SET_CAMERA, 6)),
                13 => Ok(OpcodeMeta::new(raw, Opcode::DATA_CAMERA, 2)),
                14 => Ok(OpcodeMeta::new(raw, Opcode::CHANGE_FIELD, 1)),
                15 => Ok(OpcodeMeta::new(raw, Opcode::HIDE_FIELD, 1)),
                16 => Ok(OpcodeMeta::new(raw, Opcode::MOVE_FIELD, 3)),
                17 => Ok(OpcodeMeta::new(raw, Opcode::FADEOUT_FIELD, 2)),
                18 => Ok(OpcodeMeta::new(raw, Opcode::EYE_ANIM, 3)),
                19 => Ok(OpcodeMeta::new(raw, Opcode::MOUTH_ANIM, 5)),
                20 => Ok(OpcodeMeta::new(raw, Opcode::HAND_ANIM, 5)),
                21 => Ok(OpcodeMeta::new(raw, Opcode::LOOK_ANIM, 4)),
                22 => Ok(OpcodeMeta::new(raw, Opcode::EXPRESSION, 4)),
                23 => Ok(OpcodeMeta::new(raw, Opcode::LOOK_CAMERA, 5)),
                24 => Ok(OpcodeMeta::new(raw, Opcode::LYRIC, 2)),
                25 => Ok(OpcodeMeta::new(raw, Opcode::MUSIC_PLAY, 0)),
                26 => Ok(OpcodeMeta::new(raw, Opcode::MODE_SELECT, 2)),
                27 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_MOTION, 4)),
                28 => Ok(OpcodeMeta::new(raw, Opcode::BAR_TIME_SET, 2)),
                29 => Ok(OpcodeMeta::new(raw, Opcode::SHADOWHEIGHT, 2)),
                30 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_FACE, 1)),
                31 => Ok(OpcodeMeta::new(raw, Opcode::MOVE_CAMERA, 21)),
                32 => Ok(OpcodeMeta::new(raw, Opcode::PV_END, 0)),
                33 => Ok(OpcodeMeta::new(raw, Opcode::SHADOWPOS, 3)),
                34 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_LYRIC, 2)),
                35 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_TARGET, 5)),
                36 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_MOUTH, 1)),
                37 => Ok(OpcodeMeta::new(raw, Opcode::SET_CHARA, 1)),
                38 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_MOVE, 7)),
                39 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_SHADOW, 1)),
                40 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_EYELID, 1)),
                41 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_EYE, 2)),
                42 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_ITEM, 1)),
                43 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_EFFECT, 2)),
                44 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_DISP, 1)),
                45 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_HAND_ANIM, 2)),
                46 => Ok(OpcodeMeta::new(raw, Opcode::AIM, 3)),
                47 => Ok(OpcodeMeta::new(raw, Opcode::HAND_ITEM, 3)),
                48 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_BLUSH, 1)),
                49 => Ok(OpcodeMeta::new(raw, Opcode::NEAR_CLIP, 2)),
                50 => Ok(OpcodeMeta::new(raw, Opcode::CLOTH_WET, 2)),
                51 => Ok(OpcodeMeta::new(raw, Opcode::LIGHT_ROT, 3)),
                52 => Ok(OpcodeMeta::new(raw, Opcode::SCENE_FADE, 6)),
                53 => Ok(OpcodeMeta::new(raw, Opcode::TONE_TRANS, 6)),
                54 => Ok(OpcodeMeta::new(raw, Opcode::SATURATE, 1)),
                55 => Ok(OpcodeMeta::new(raw, Opcode::FADE_MODE, 1)),
                56 => Ok(OpcodeMeta::new(raw, Opcode::AUTO_BLINK, 2)),
                57 => Ok(OpcodeMeta::new(raw, Opcode::PARTS_DISP, 3)),
                58 => Ok(OpcodeMeta::new(raw, Opcode::TARGET_FLYING_TIME, 1)),
                59 => Ok(OpcodeMeta::new(raw, Opcode::CHARA_SIZE, 2)),
                60 => Ok(OpcodeMeta::new(raw, Opcode::CHARA_HEIGHT_ADJUST, 2)),
                61 => Ok(OpcodeMeta::new(raw, Opcode::ITEM_ANIM, 4)),
                62 => Ok(OpcodeMeta::new(raw, Opcode::CHARA_POS_ADJUST, 4)),
                63 => Ok(OpcodeMeta::new(raw, Opcode::SCENE_ROT, 1)),
                64 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_MOT_SMOOTH_LEN, 2)),
                65 => Ok(OpcodeMeta::new(raw, Opcode::PV_BRANCH_MODE, 1)),
                66 => Ok(OpcodeMeta::new(raw, Opcode::DATA_CAMERA_START, 2)),
                67 => Ok(OpcodeMeta::new(raw, Opcode::MOVIE_PLAY, 1)),
                68 => Ok(OpcodeMeta::new(raw, Opcode::MOVIE_DISP, 1)),
                69 => Ok(OpcodeMeta::new(raw, Opcode::WIND, 3)),
                70 => Ok(OpcodeMeta::new(raw, Opcode::OSAGE_STEP, 3)),
                71 => Ok(OpcodeMeta::new(raw, Opcode::OSAGE_MV_CCL, 3)),
                72 => Ok(OpcodeMeta::new(raw, Opcode::CHARA_COLOR, 2)),
                73 => Ok(OpcodeMeta::new(raw, Opcode::SE_EFFECT, 1)),
                74 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_MOVE_XYZ, 9)),
                75 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_EYELID_ANIM, 3)),
                76 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_INSTRUMENT_ITEM, 2)),
                77 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_MOTION_LOOP, 4)),
                78 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_EXPRESSION, 2)),
                79 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_EYE_ANIM, 3)),
                80 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_MOUTH_ANIM, 2)),
                81 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_CAMERA, 24)),
                82 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_MODE_SELECT, 1)),
                83 => Ok(OpcodeMeta::new(raw, Opcode::PV_END_FADEOUT, 2)),
                _ => Err(ApplicationError::UnknownOpcode(raw)),
            },
            Game::F2nd => match raw {
                0 => Ok(OpcodeMeta::new(raw, Opcode::END, 0)),
                1 => Ok(OpcodeMeta::new(raw, Opcode::TIME, 1)),
                2 => Ok(OpcodeMeta::new(raw, Opcode::MIKU_MOVE, 4)),
                3 => Ok(OpcodeMeta::new(raw, Opcode::MIKU_ROT, 2)),
                4 => Ok(OpcodeMeta::new(raw, Opcode::MIKU_DISP, 2)),
                5 => Ok(OpcodeMeta::new(raw, Opcode::MIKU_SHADOW, 2)),
                6 => Ok(OpcodeMeta::new(raw, Opcode::TARGET, 12)),
                7 => Ok(OpcodeMeta::new(raw, Opcode::SET_MOTION, 4)),
                8 => Ok(OpcodeMeta::new(raw, Opcode::SET_PLAYDATA, 2)),
                9 => Ok(OpcodeMeta::new(raw, Opcode::EFFECT, 6)),
                10 => Ok(OpcodeMeta::new(raw, Opcode::FADEIN_FIELD, 2)),
                11 => Ok(OpcodeMeta::new(raw, Opcode::EFFECT_OFF, 1)),
                12 => Ok(OpcodeMeta::new(raw, Opcode::SET_CAMERA, 6)),
                13 => Ok(OpcodeMeta::new(raw, Opcode::DATA_CAMERA, 2)),
                14 => Ok(OpcodeMeta::new(raw, Opcode::CHANGE_FIELD, 2)),
                15 => Ok(OpcodeMeta::new(raw, Opcode::HIDE_FIELD, 1)),
                16 => Ok(OpcodeMeta::new(raw, Opcode::MOVE_FIELD, 3)),
                17 => Ok(OpcodeMeta::new(raw, Opcode::FADEOUT_FIELD, 2)),
                18 => Ok(OpcodeMeta::new(raw, Opcode::EYE_ANIM, 3)),
                19 => Ok(OpcodeMeta::new(raw, Opcode::MOUTH_ANIM, 5)),
                20 => Ok(OpcodeMeta::new(raw, Opcode::HAND_ANIM, 5)),
                21 => Ok(OpcodeMeta::new(raw, Opcode::LOOK_ANIM, 4)),
                22 => Ok(OpcodeMeta::new(raw, Opcode::EXPRESSION, 4)),
                23 => Ok(OpcodeMeta::new(raw, Opcode::LOOK_CAMERA, 5)),
                24 => Ok(OpcodeMeta::new(raw, Opcode::LYRIC, 2)),
                25 => Ok(OpcodeMeta::new(raw, Opcode::MUSIC_PLAY, 0)),
                26 => Ok(OpcodeMeta::new(raw, Opcode::MODE_SELECT, 2)),
                27 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_MOTION, 4)),
                28 => Ok(OpcodeMeta::new(raw, Opcode::BAR_TIME_SET, 2)),
                29 => Ok(OpcodeMeta::new(raw, Opcode::SHADOWHEIGHT, 2)),
                30 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_FACE, 1)),
                31 => Ok(OpcodeMeta::new(raw, Opcode::MOVE_CAMERA, 21)),
                32 => Ok(OpcodeMeta::new(raw, Opcode::PV_END, 0)),
                33 => Ok(OpcodeMeta::new(raw, Opcode::SHADOWPOS, 3)),
                34 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_LYRIC, 2)),
                35 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_TARGET, 5)),
                36 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_MOUTH, 1)),
                37 => Ok(OpcodeMeta::new(raw, Opcode::SET_CHARA, 1)),
                38 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_MOVE, 7)),
                39 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_SHADOW, 1)),
                40 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_EYELID, 1)),
                41 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_EYE, 2)),
                42 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_ITEM, 1)),
                43 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_EFFECT, 2)),
                44 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_DISP, 1)),
                45 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_HAND_ANIM, 2)),
                46 => Ok(OpcodeMeta::new(raw, Opcode::AIM, 3)),
                47 => Ok(OpcodeMeta::new(raw, Opcode::HAND_ITEM, 3)),
                48 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_BLUSH, 1)),
                49 => Ok(OpcodeMeta::new(raw, Opcode::NEAR_CLIP, 2)),
                50 => Ok(OpcodeMeta::new(raw, Opcode::CLOTH_WET, 2)),
                51 => Ok(OpcodeMeta::new(raw, Opcode::LIGHT_ROT, 3)),
                52 => Ok(OpcodeMeta::new(raw, Opcode::SCENE_FADE, 6)),
                53 => Ok(OpcodeMeta::new(raw, Opcode::TONE_TRANS, 6)),
                54 => Ok(OpcodeMeta::new(raw, Opcode::SATURATE, 1)),
                55 => Ok(OpcodeMeta::new(raw, Opcode::FADE_MODE, 1)),
                56 => Ok(OpcodeMeta::new(raw, Opcode::AUTO_BLINK, 2)),
                57 => Ok(OpcodeMeta::new(raw, Opcode::PARTS_DISP, 3)),
                58 => Ok(OpcodeMeta::new(raw, Opcode::TARGET_FLYING_TIME, 1)),
                59 => Ok(OpcodeMeta::new(raw, Opcode::CHARA_SIZE, 2)),
                60 => Ok(OpcodeMeta::new(raw, Opcode::CHARA_HEIGHT_ADJUST, 2)),
                61 => Ok(OpcodeMeta::new(raw, Opcode::ITEM_ANIM, 4)),
                62 => Ok(OpcodeMeta::new(raw, Opcode::CHARA_POS_ADJUST, 4)),
                63 => Ok(OpcodeMeta::new(raw, Opcode::SCENE_ROT, 1)),
                64 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_MOT_SMOOTH_LEN, 2)),
                65 => Ok(OpcodeMeta::new(raw, Opcode::PV_BRANCH_MODE, 1)),
                66 => Ok(OpcodeMeta::new(raw, Opcode::DATA_CAMERA_START, 2)),
                67 => Ok(OpcodeMeta::new(raw, Opcode::MOVIE_PLAY, 1)),
                68 => Ok(OpcodeMeta::new(raw, Opcode::MOVIE_DISP, 1)),
                69 => Ok(OpcodeMeta::new(raw, Opcode::WIND, 3)),
                70 => Ok(OpcodeMeta::new(raw, Opcode::OSAGE_STEP, 3)),
                71 => Ok(OpcodeMeta::new(raw, Opcode::OSAGE_MV_CCL, 3)),
                72 => Ok(OpcodeMeta::new(raw, Opcode::CHARA_COLOR, 2)),
                73 => Ok(OpcodeMeta::new(raw, Opcode::SE_EFFECT, 1)),
                74 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_MOVE_XYZ, 9)),
                75 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_EYELID_ANIM, 3)),
                76 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_INSTRUMENT_ITEM, 2)),
                77 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_MOTION_LOOP, 4)),
                78 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_EXPRESSION, 2)),
                79 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_EYE_ANIM, 3)),
                80 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_MOUTH_ANIM, 2)),
                81 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_CAMERA, 22)),
                82 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_MODE_SELECT, 1)),
                83 => Ok(OpcodeMeta::new(raw, Opcode::PV_END_FADEOUT, 2)),
                87 => Ok(OpcodeMeta::new(raw, Opcode::RESERVE, 9)),
                88 => Ok(OpcodeMeta::new(raw, Opcode::PV_AUTH_LIGHT_PRIORITY, 2)),
                89 => Ok(OpcodeMeta::new(raw, Opcode::PV_CHARA_LIGHT, 3)),
                90 => Ok(OpcodeMeta::new(raw, Opcode::PV_STAGE_LIGHT, 3)),
                91 => Ok(OpcodeMeta::new(raw, Opcode::TARGET_EFFECT, 11)),
                92 => Ok(OpcodeMeta::new(raw, Opcode::FOG, 3)),
                93 => Ok(OpcodeMeta::new(raw, Opcode::BLOOM, 2)),
                94 => Ok(OpcodeMeta::new(raw, Opcode::COLOR_CORRECTION, 3)),
                95 => Ok(OpcodeMeta::new(raw, Opcode::DOF, 3)),
                96 => Ok(OpcodeMeta::new(raw, Opcode::CHARA_ALPHA, 4)),
                97 => Ok(OpcodeMeta::new(raw, Opcode::AUTO_CAPTURE_BEGIN, 1)),
                98 => Ok(OpcodeMeta::new(raw, Opcode::MANUAL_CAPTURE, 1)),
                99 => Ok(OpcodeMeta::new(raw, Opcode::TOON_EDGE, 3)),
                100 => Ok(OpcodeMeta::new(raw, Opcode::SHIMMER, 3)),
                101 => Ok(OpcodeMeta::new(raw, Opcode::ITEM_ALPHA, 4)),
                102 => Ok(OpcodeMeta::new(raw, Opcode::MOVIE_CUT, 1)),
                103 => Ok(OpcodeMeta::new(raw, Opcode::CROSSFADE, 1)),
                104 => Ok(OpcodeMeta::new(raw, Opcode::SUBFRAMERENDER, 1)),
                105 => Ok(OpcodeMeta::new(raw, Opcode::EVENT_JUDGE, 36)),
                106 => Ok(OpcodeMeta::new(raw, Opcode::TOON???EDGE, 2)),
                107 => Ok(OpcodeMeta::new(raw, Opcode::FOG_ENABLE, 2)),
                108 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_CAMERA_BOX, 112)),
                109 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_STAGE_PARAM, 1)),
                110 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_CHANGE_FIELD, 1)),
                _ => Err(ApplicationError::UnknownOpcode(raw)),
            },
            Game::X => match raw {
                0 => Ok(OpcodeMeta::new(raw, Opcode::END, 0)),
                1 => Ok(OpcodeMeta::new(raw, Opcode::TIME, 1)),
                2 => Ok(OpcodeMeta::new(raw, Opcode::MIKU_MOVE, 4)),
                3 => Ok(OpcodeMeta::new(raw, Opcode::MIKU_ROT, 2)),
                4 => Ok(OpcodeMeta::new(raw, Opcode::MIKU_DISP, 2)),
                5 => Ok(OpcodeMeta::new(raw, Opcode::MIKU_SHADOW, 2)),
                6 => Ok(OpcodeMeta::new(raw, Opcode::TARGET, 12)),
                7 => Ok(OpcodeMeta::new(raw, Opcode::SET_MOTION, 4)),
                8 => Ok(OpcodeMeta::new(raw, Opcode::SET_PLAYDATA, 2)),
                9 => Ok(OpcodeMeta::new(raw, Opcode::EFFECT, 6)),
                10 => Ok(OpcodeMeta::new(raw, Opcode::FADEIN_FIELD, 2)),
                11 => Ok(OpcodeMeta::new(raw, Opcode::EFFECT_OFF, 1)),
                12 => Ok(OpcodeMeta::new(raw, Opcode::SET_CAMERA, 6)),
                13 => Ok(OpcodeMeta::new(raw, Opcode::DATA_CAMERA, 2)),
                14 => Ok(OpcodeMeta::new(raw, Opcode::CHANGE_FIELD, 2)),
                15 => Ok(OpcodeMeta::new(raw, Opcode::HIDE_FIELD, 1)),
                16 => Ok(OpcodeMeta::new(raw, Opcode::MOVE_FIELD, 3)),
                17 => Ok(OpcodeMeta::new(raw, Opcode::FADEOUT_FIELD, 2)),
                18 => Ok(OpcodeMeta::new(raw, Opcode::EYE_ANIM, 3)),
                19 => Ok(OpcodeMeta::new(raw, Opcode::MOUTH_ANIM, 5)),
                20 => Ok(OpcodeMeta::new(raw, Opcode::HAND_ANIM, 5)),
                21 => Ok(OpcodeMeta::new(raw, Opcode::LOOK_ANIM, 4)),
                22 => Ok(OpcodeMeta::new(raw, Opcode::EXPRESSION, 4)),
                23 => Ok(OpcodeMeta::new(raw, Opcode::LOOK_CAMERA, 5)),
                24 => Ok(OpcodeMeta::new(raw, Opcode::LYRIC, 2)),
                25 => Ok(OpcodeMeta::new(raw, Opcode::MUSIC_PLAY, 0)),
                26 => Ok(OpcodeMeta::new(raw, Opcode::MODE_SELECT, 2)),
                27 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_MOTION, 4)),
                28 => Ok(OpcodeMeta::new(raw, Opcode::BAR_TIME_SET, 2)),
                29 => Ok(OpcodeMeta::new(raw, Opcode::SHADOWHEIGHT, 2)),
                30 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_FACE, 1)),
                31 => Ok(OpcodeMeta::new(raw, Opcode::DUMMY, 21)),
                32 => Ok(OpcodeMeta::new(raw, Opcode::PV_END, 0)),
                33 => Ok(OpcodeMeta::new(raw, Opcode::SHADOWPOS, 3)),
                34 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_LYRIC, 2)),
                35 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_TARGET, 5)),
                36 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_MOUTH, 1)),
                37 => Ok(OpcodeMeta::new(raw, Opcode::SET_CHARA, 1)),
                38 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_MOVE, 7)),
                39 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_SHADOW, 1)),
                40 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_EYELID, 1)),
                41 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_EYE, 2)),
                42 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_ITEM, 1)),
                43 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_EFFECT, 2)),
                44 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_DISP, 1)),
                45 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_HAND_ANIM, 2)),
                46 => Ok(OpcodeMeta::new(raw, Opcode::AIM, 3)),
                47 => Ok(OpcodeMeta::new(raw, Opcode::HAND_ITEM, 3)),
                48 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_BLUSH, 1)),
                49 => Ok(OpcodeMeta::new(raw, Opcode::NEAR_CLIP, 2)),
                50 => Ok(OpcodeMeta::new(raw, Opcode::CLOTH_WET, 2)),
                51 => Ok(OpcodeMeta::new(raw, Opcode::LIGHT_ROT, 3)),
                52 => Ok(OpcodeMeta::new(raw, Opcode::SCENE_FADE, 6)),
                53 => Ok(OpcodeMeta::new(raw, Opcode::TONE_TRANS, 6)),
                54 => Ok(OpcodeMeta::new(raw, Opcode::SATURATE, 1)),
                55 => Ok(OpcodeMeta::new(raw, Opcode::FADE_MODE, 1)),
                56 => Ok(OpcodeMeta::new(raw, Opcode::AUTO_BLINK, 2)),
                57 => Ok(OpcodeMeta::new(raw, Opcode::PARTS_DISP, 3)),
                58 => Ok(OpcodeMeta::new(raw, Opcode::TARGET_FLYING_TIME, 1)),
                59 => Ok(OpcodeMeta::new(raw, Opcode::CHARA_SIZE, 2)),
                60 => Ok(OpcodeMeta::new(raw, Opcode::CHARA_HEIGHT_ADJUST, 2)),
                61 => Ok(OpcodeMeta::new(raw, Opcode::ITEM_ANIM, 4)),
                62 => Ok(OpcodeMeta::new(raw, Opcode::CHARA_POS_ADJUST, 4)),
                63 => Ok(OpcodeMeta::new(raw, Opcode::SCENE_ROT, 1)),
                64 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_MOT_SMOOTH_LEN, 2)),
                65 => Ok(OpcodeMeta::new(raw, Opcode::PV_BRANCH_MODE, 1)),
                66 => Ok(OpcodeMeta::new(raw, Opcode::DATA_CAMERA_START, 2)),
                67 => Ok(OpcodeMeta::new(raw, Opcode::MOVIE_PLAY, 1)),
                68 => Ok(OpcodeMeta::new(raw, Opcode::MOVIE_DISP, 1)),
                69 => Ok(OpcodeMeta::new(raw, Opcode::WIND, 3)),
                70 => Ok(OpcodeMeta::new(raw, Opcode::OSAGE_STEP, 3)),
                71 => Ok(OpcodeMeta::new(raw, Opcode::OSAGE_MV_CCL, 3)),
                72 => Ok(OpcodeMeta::new(raw, Opcode::CHARA_COLOR, 2)),
                73 => Ok(OpcodeMeta::new(raw, Opcode::SE_EFFECT, 1)),
                74 => Ok(OpcodeMeta::new(raw, Opcode::CHARA_SHADOW_QUALITY, 2)),
                75 => Ok(OpcodeMeta::new(raw, Opcode::STAGE_SHADOW_QUALITY, 2)),
                76 => Ok(OpcodeMeta::new(raw, Opcode::COMMON_LIGHT, 2)),
                77 => Ok(OpcodeMeta::new(raw, Opcode::TONE_MAP, 2)),
                78 => Ok(OpcodeMeta::new(raw, Opcode::IBL_COLOR, 2)),
                79 => Ok(OpcodeMeta::new(raw, Opcode::REFLECTION, 2)),
                80 => Ok(OpcodeMeta::new(raw, Opcode::CHROMATIC_ABERRATION, 3)),
                81 => Ok(OpcodeMeta::new(raw, Opcode::STAGE_SHADOW, 2)),
                82 => Ok(OpcodeMeta::new(raw, Opcode::REFLECTION_QUALITY, 2)),
                83 => Ok(OpcodeMeta::new(raw, Opcode::PV_END_FADEOUT, 2)),
                84 => Ok(OpcodeMeta::new(raw, Opcode::CREDIT_TITLE, 1)),
                85 => Ok(OpcodeMeta::new(raw, Opcode::BAR_POINT, 1)),
                86 => Ok(OpcodeMeta::new(raw, Opcode::BEAT_POINT, 1)),
                88 => Ok(OpcodeMeta::new(raw, Opcode::PV_AUTH_LIGHT_PRIORITY, 2)),
                89 => Ok(OpcodeMeta::new(raw, Opcode::PV_CHARA_LIGHT, 3)),
                90 => Ok(OpcodeMeta::new(raw, Opcode::PV_STAGE_LIGHT, 3)),
                91 => Ok(OpcodeMeta::new(raw, Opcode::TARGET_EFFECT, 11)),
                92 => Ok(OpcodeMeta::new(raw, Opcode::FOG, 3)),
                93 => Ok(OpcodeMeta::new(raw, Opcode::BLOOM, 2)),
                94 => Ok(OpcodeMeta::new(raw, Opcode::COLOR_CORRECTION, 3)),
                95 => Ok(OpcodeMeta::new(raw, Opcode::DOF, 3)),
                96 => Ok(OpcodeMeta::new(raw, Opcode::CHARA_ALPHA, 4)),
                97 => Ok(OpcodeMeta::new(raw, Opcode::AUTO_CAPTURE_BEGIN, 1)),
                98 => Ok(OpcodeMeta::new(raw, Opcode::MANUAL_CAPTURE, 1)),
                99 => Ok(OpcodeMeta::new(raw, Opcode::TOON_EDGE, 3)),
                100 => Ok(OpcodeMeta::new(raw, Opcode::SHIMMER, 3)),
                101 => Ok(OpcodeMeta::new(raw, Opcode::ITEM_ALPHA, 4)),
                102 => Ok(OpcodeMeta::new(raw, Opcode::MOVIE_CUT, 1)),
                103 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_CAMERA_BOX, 112)),
                104 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_STAGE_PARAM, 1)),
                105 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_CHANGE_FIELD, 1)),
                106 => Ok(OpcodeMeta::new(raw, Opcode::MIKUDAYO_ADJUST, 7)),
                107 => Ok(OpcodeMeta::new(raw, Opcode::LYRIC_2, 2)),
                108 => Ok(OpcodeMeta::new(raw, Opcode::LYRIC_READ, 2)),
                109 => Ok(OpcodeMeta::new(raw, Opcode::LYRIC_READ_2, 2)),
                110 => Ok(OpcodeMeta::new(raw, Opcode::ANNOTATION, 5)),
                111 => Ok(OpcodeMeta::new(raw, Opcode::STAGE_EFFECT, 2)),
                112 => Ok(OpcodeMeta::new(raw, Opcode::SONG_EFFECT, 3)),
                113 => Ok(OpcodeMeta::new(raw, Opcode::SONG_EFFECT_ATTACH, 3)),
                114 => Ok(OpcodeMeta::new(raw, Opcode::LIGHT_AUTH, 2)),
                115 => Ok(OpcodeMeta::new(raw, Opcode::FADE, 2)),
                116 => Ok(OpcodeMeta::new(raw, Opcode::SET_STAGE_EFFECT_ENV, 2)),
                117 => Ok(OpcodeMeta::new(raw, Opcode::RESERVE, 2)),
                118 => Ok(OpcodeMeta::new(raw, Opcode::COMMON_EFFECT_AET_FRONT, 2)),
                119 => Ok(OpcodeMeta::new(raw, Opcode::COMMON_EFFECT_AET_FRONT_LOW, 2)),
                120 => Ok(OpcodeMeta::new(raw, Opcode::COMMON_EFFECT_PARTICLE, 2)),
                121 => Ok(OpcodeMeta::new(raw, Opcode::SONG_EFFECT_ALPHA_SORT, 3)),
                122 => Ok(OpcodeMeta::new(raw, Opcode::LOOK_CAMERA_FACE_LIMIT, 5)),
                123 => Ok(OpcodeMeta::new(raw, Opcode::ITEM_LIGHT, 3)),
                124 => Ok(OpcodeMeta::new(raw, Opcode::CHARA_EFFECT, 3)),
                125 => Ok(OpcodeMeta::new(raw, Opcode::MARKER, 2)),
                126 => Ok(OpcodeMeta::new(raw, Opcode::CHARA_EFFECT_CHARA_LIGHT, 3)),
                127 => Ok(OpcodeMeta::new(
                    raw,
                    Opcode::ENABLE_COMMON_LIGHT_TO_CHARA,
                    2,
                )),
                128 => Ok(OpcodeMeta::new(raw, Opcode::ENABLE_FXAA, 2)),
                129 => Ok(OpcodeMeta::new(raw, Opcode::ENABLE_TEMPORAL_AA, 2)),
                130 => Ok(OpcodeMeta::new(raw, Opcode::ENABLE_REFLECTION, 2)),
                131 => Ok(OpcodeMeta::new(raw, Opcode::BANK_BRANCH, 2)),
                132 => Ok(OpcodeMeta::new(raw, Opcode::BANK_END, 2)),
                141 => Ok(OpcodeMeta::new(raw, Opcode::VR_LIVE_MOVIE, 2)),
                142 => Ok(OpcodeMeta::new(raw, Opcode::VR_CHEER, 2)),
                143 => Ok(OpcodeMeta::new(raw, Opcode::VR_CHARA_PSMOVE, 2)),
                144 => Ok(OpcodeMeta::new(raw, Opcode::VR_MOVE_PATH, 2)),
                145 => Ok(OpcodeMeta::new(raw, Opcode::VR_SET_BASE, 2)),
                146 => Ok(OpcodeMeta::new(raw, Opcode::VR_TECH_DEMO_EFFECT, 2)),
                147 => Ok(OpcodeMeta::new(raw, Opcode::VR_TRANSFORM, 2)),
                148 => Ok(OpcodeMeta::new(raw, Opcode::GAZE, 2)),
                149 => Ok(OpcodeMeta::new(raw, Opcode::TECH_DEMO_GESUTRE, 2)),
                150 => Ok(OpcodeMeta::new(raw, Opcode::VR_CHEMICAL_LIGHT_COLOR, 2)),
                151 => Ok(OpcodeMeta::new(raw, Opcode::VR_LIVE_MOB, 5)),
                152 => Ok(OpcodeMeta::new(raw, Opcode::VR_LIVE_HAIR_OSAGE, 9)),
                153 => Ok(OpcodeMeta::new(raw, Opcode::VR_LIVE_LOOK_CAMERA, 9)),
                154 => Ok(OpcodeMeta::new(raw, Opcode::VR_LIVE_CHEER, 5)),
                155 => Ok(OpcodeMeta::new(raw, Opcode::VR_LIVE_GESTURE, 3)),
                156 => Ok(OpcodeMeta::new(raw, Opcode::VR_LIVE_CLONE, 7)),
                157 => Ok(OpcodeMeta::new(raw, Opcode::VR_LOOP_EFFECT, 7)),
                158 => Ok(OpcodeMeta::new(raw, Opcode::VR_LIVE_ONESHOT_EFFECT, 6)),
                159 => Ok(OpcodeMeta::new(raw, Opcode::VR_LIVE_PRESENT, 9)),
                160 => Ok(OpcodeMeta::new(raw, Opcode::VR_LIVE_TRANSFORM, 5)),
                161 => Ok(OpcodeMeta::new(raw, Opcode::VR_LIVE_FLY, 5)),
                162 => Ok(OpcodeMeta::new(raw, Opcode::VR_LIVE_CHARA_VOICE, 2)),
                _ => Err(ApplicationError::UnknownOpcode(raw)),
            },
            Game::FutureTone => match raw {
                0 => Ok(OpcodeMeta::new(raw, Opcode::END, 0)),
                1 => Ok(OpcodeMeta::new(raw, Opcode::TIME, 1)),
                2 => Ok(OpcodeMeta::new(raw, Opcode::MIKU_MOVE, 4)),
                3 => Ok(OpcodeMeta::new(raw, Opcode::MIKU_ROT, 2)),
                4 => Ok(OpcodeMeta::new(raw, Opcode::MIKU_DISP, 2)),
                5 => Ok(OpcodeMeta::new(raw, Opcode::MIKU_SHADOW, 2)),
                6 => Ok(OpcodeMeta::new(raw, Opcode::TARGET, 7)),
                7 => Ok(OpcodeMeta::new(raw, Opcode::SET_MOTION, 4)),
                8 => Ok(OpcodeMeta::new(raw, Opcode::SET_PLAYDATA, 2)),
                9 => Ok(OpcodeMeta::new(raw, Opcode::EFFECT, 6)),
                10 => Ok(OpcodeMeta::new(raw, Opcode::FADEIN_FIELD, 2)),
                11 => Ok(OpcodeMeta::new(raw, Opcode::EFFECT_OFF, 1)),
                12 => Ok(OpcodeMeta::new(raw, Opcode::SET_CAMERA, 6)),
                13 => Ok(OpcodeMeta::new(raw, Opcode::DATA_CAMERA, 2)),
                14 => Ok(OpcodeMeta::new(raw, Opcode::CHANGE_FIELD, 1)),
                15 => Ok(OpcodeMeta::new(raw, Opcode::HIDE_FIELD, 1)),
                16 => Ok(OpcodeMeta::new(raw, Opcode::MOVE_FIELD, 3)),
                17 => Ok(OpcodeMeta::new(raw, Opcode::FADEOUT_FIELD, 2)),
                18 => Ok(OpcodeMeta::new(raw, Opcode::EYE_ANIM, 3)),
                19 => Ok(OpcodeMeta::new(raw, Opcode::MOUTH_ANIM, 5)),
                20 => Ok(OpcodeMeta::new(raw, Opcode::HAND_ANIM, 5)),
                21 => Ok(OpcodeMeta::new(raw, Opcode::LOOK_ANIM, 4)),
                22 => Ok(OpcodeMeta::new(raw, Opcode::EXPRESSION, 4)),
                23 => Ok(OpcodeMeta::new(raw, Opcode::LOOK_CAMERA, 5)),
                24 => Ok(OpcodeMeta::new(raw, Opcode::LYRIC, 2)),
                25 => Ok(OpcodeMeta::new(raw, Opcode::MUSIC_PLAY, 0)),
                26 => Ok(OpcodeMeta::new(raw, Opcode::MODE_SELECT, 2)),
                27 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_MOTION, 4)),
                28 => Ok(OpcodeMeta::new(raw, Opcode::BAR_TIME_SET, 2)),
                29 => Ok(OpcodeMeta::new(raw, Opcode::SHADOWHEIGHT, 2)),
                30 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_FACE, 1)),
                31 => Ok(OpcodeMeta::new(raw, Opcode::MOVE_CAMERA, 21)),
                32 => Ok(OpcodeMeta::new(raw, Opcode::PV_END, 0)),
                33 => Ok(OpcodeMeta::new(raw, Opcode::SHADOWPOS, 3)),
                34 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_LYRIC, 2)),
                35 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_TARGET, 5)),
                36 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_MOUTH, 1)),
                37 => Ok(OpcodeMeta::new(raw, Opcode::SET_CHARA, 1)),
                38 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_MOVE, 7)),
                39 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_SHADOW, 1)),
                40 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_EYELID, 1)),
                41 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_EYE, 2)),
                42 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_ITEM, 1)),
                43 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_EFFECT, 2)),
                44 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_DISP, 1)),
                45 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_HAND_ANIM, 2)),
                46 => Ok(OpcodeMeta::new(raw, Opcode::AIM, 3)),
                47 => Ok(OpcodeMeta::new(raw, Opcode::HAND_ITEM, 3)),
                48 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_BLUSH, 1)),
                49 => Ok(OpcodeMeta::new(raw, Opcode::NEAR_CLIP, 2)),
                50 => Ok(OpcodeMeta::new(raw, Opcode::CLOTH_WET, 2)),
                51 => Ok(OpcodeMeta::new(raw, Opcode::LIGHT_ROT, 3)),
                52 => Ok(OpcodeMeta::new(raw, Opcode::SCENE_FADE, 6)),
                53 => Ok(OpcodeMeta::new(raw, Opcode::TONE_TRANS, 6)),
                54 => Ok(OpcodeMeta::new(raw, Opcode::SATURATE, 1)),
                55 => Ok(OpcodeMeta::new(raw, Opcode::FADE_MODE, 1)),
                56 => Ok(OpcodeMeta::new(raw, Opcode::AUTO_BLINK, 2)),
                57 => Ok(OpcodeMeta::new(raw, Opcode::PARTS_DISP, 3)),
                58 => Ok(OpcodeMeta::new(raw, Opcode::TARGET_FLYING_TIME, 1)),
                59 => Ok(OpcodeMeta::new(raw, Opcode::CHARA_SIZE, 2)),
                60 => Ok(OpcodeMeta::new(raw, Opcode::CHARA_HEIGHT_ADJUST, 2)),
                61 => Ok(OpcodeMeta::new(raw, Opcode::ITEM_ANIM, 4)),
                62 => Ok(OpcodeMeta::new(raw, Opcode::CHARA_POS_ADJUST, 4)),
                63 => Ok(OpcodeMeta::new(raw, Opcode::SCENE_ROT, 1)),
                64 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_MOT_SMOOTH_LEN, 2)),
                65 => Ok(OpcodeMeta::new(raw, Opcode::PV_BRANCH_MODE, 1)),
                66 => Ok(OpcodeMeta::new(raw, Opcode::DATA_CAMERA_START, 2)),
                67 => Ok(OpcodeMeta::new(raw, Opcode::MOVIE_PLAY, 1)),
                68 => Ok(OpcodeMeta::new(raw, Opcode::MOVIE_DISP, 1)),
                69 => Ok(OpcodeMeta::new(raw, Opcode::WIND, 3)),
                70 => Ok(OpcodeMeta::new(raw, Opcode::OSAGE_STEP, 3)),
                71 => Ok(OpcodeMeta::new(raw, Opcode::OSAGE_MV_CCL, 3)),
                72 => Ok(OpcodeMeta::new(raw, Opcode::CHARA_COLOR, 2)),
                73 => Ok(OpcodeMeta::new(raw, Opcode::SE_EFFECT, 1)),
                74 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_MOVE_XYZ, 9)),
                75 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_EYELID_ANIM, 3)),
                76 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_INSTRUMENT_ITEM, 2)),
                77 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_MOTION_LOOP, 4)),
                78 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_EXPRESSION, 2)),
                79 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_EYE_ANIM, 3)),
                80 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_MOUTH_ANIM, 2)),
                81 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_CAMERA, 24)),
                82 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_MODE_SELECT, 1)),
                83 => Ok(OpcodeMeta::new(raw, Opcode::PV_END_FADEOUT, 2)),
                84 => Ok(OpcodeMeta::new(raw, Opcode::TARGET_FLAG, 1)),
                85 => Ok(OpcodeMeta::new(raw, Opcode::ITEM_ANIM_ATTACH, 3)),
                86 => Ok(OpcodeMeta::new(raw, Opcode::SHADOW_RANGE, 1)),
                87 => Ok(OpcodeMeta::new(raw, Opcode::HAND_SCALE, 3)),
                88 => Ok(OpcodeMeta::new(raw, Opcode::LIGHT_POS, 4)),
                89 => Ok(OpcodeMeta::new(raw, Opcode::FACE_TYPE, 1)),
                90 => Ok(OpcodeMeta::new(raw, Opcode::SHADOW_CAST, 2)),
                91 => Ok(OpcodeMeta::new(raw, Opcode::EDIT_MOTION_F, 6)),
                92 => Ok(OpcodeMeta::new(raw, Opcode::FOG, 3)),
                93 => Ok(OpcodeMeta::new(raw, Opcode::BLOOM, 2)),
                94 => Ok(OpcodeMeta::new(raw, Opcode::COLOR_COLLE, 3)),
                95 => Ok(OpcodeMeta::new(raw, Opcode::DOF, 3)),
                96 => Ok(OpcodeMeta::new(raw, Opcode::CHARA_ALPHA, 4)),
                97 => Ok(OpcodeMeta::new(raw, Opcode::AOTO_CAP, 1)),
                98 => Ok(OpcodeMeta::new(raw, Opcode::MAN_CAP, 1)),
                99 => Ok(OpcodeMeta::new(raw, Opcode::TOON, 3)),
                100 => Ok(OpcodeMeta::new(raw, Opcode::SHIMMER, 3)),
                101 => Ok(OpcodeMeta::new(raw, Opcode::ITEM_ALPHA, 4)),
                102 => Ok(OpcodeMeta::new(raw, Opcode::MOVIE_CUT_CHG, 1)),
                103 => Ok(OpcodeMeta::new(raw, Opcode::CHARA_LIGHT, 3)),
                104 => Ok(OpcodeMeta::new(raw, Opcode::STAGE_LIGHT, 3)),
                105 => Ok(OpcodeMeta::new(raw, Opcode::AGEAGE_CTRL, 8)),
                106 => Ok(OpcodeMeta::new(raw, Opcode::PSE, 2)),
                _ => Err(ApplicationError::UnknownOpcode(raw)),
            },
            _ => Err(ApplicationError::UnsupportedGame(game)),
        }
    }

    pub fn get_opcode_meta_from_name(game: Game, name: String) -> ApplicationResult<OpcodeMeta> {
        let to = match game {
            Game::F => 83,
            Game::F2nd => 110,
            Game::X => 162,
            Game::FutureTone => 106,
            _ => 0,
        };

        for i in 0..to {
            let meta = Command::get_opcode_meta(game, i);

            match meta {
                Ok(m) => {
                    if format!("{:?}", m.opcode) == name {
                        return Ok(m);
                    }
                }
                Err(_) => {}
            }
        }

        Err(ApplicationError::UnknownOpcodeName(name))
    }
}

impl std::cmp::PartialEq for Command {
    fn eq(&self, other: &Self) -> bool {
        self.meta.opcode == other.meta.opcode && self.args == other.args
    }
}
