pub struct JSONSaveDBRow {
    id: i64,
    campaign_id: i64,
    name: String,

    sort_order: i64,
    type: String,
    save_type: String,
    export_generic_json: String,
    export_generic_json_length?: i64,
    user_is_wildcard?: bool,

    setting_name?: String,

    shareurl: String,
    short_desc: String,

    share_public: bool ,
    share_copy: bool,

    imageurl: String,
    imagetokenurl: String,
    imagesettingurl: String,
    folder: String,

    created_by: i64,
    created_on: String,

    rifts_living_campaign: bool,

    updated_by: i64,
    updated_on: String,

    share_html: String,

    hits: i64,
    total_hits: i64,

    data: String,

    deleted: bool,
    deleted_on: Date,
    deleted_by: i64,

    show_character_sheet: bool,
    allow_download: bool,

    session_id: i64,
    co_owner: i64,
    co_owner_folder: String,
    // co_owner_public: IPublicUserInfo,
}