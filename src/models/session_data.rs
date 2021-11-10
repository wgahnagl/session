pub struct Session {
    pub home: HomeValues,         // the home location of the user
    pub look_at: (i64, i64, i64), // the direction the avatar should be facing
    // This is a unit vector so
    // (0, 1, 0) is facing straight north,
    // (1, 0, 0) is east,
    // (0,-1, 0) is south and
    // (-1, 0, 0) is west.
    pub agent_access: AgentAccess, // The current maturity access level of the user
    pub agent_access_max: AgentAccess, // THe maximum level of region the user can access
    pub seed_capability: String, // The URL that the viewer should use to request further capabilities
    pub first_name: String,      // The first name of the user
    pub last_name: String,       // The last name of the user
    pub agent_id: String,        // The id of the user
    pub sim_ip: String,          // The ip used to communicate with the recieving simulator
    pub sim_port: String,        // The UDP port used to communicate with the receiving simulator
    pub http_port: String,       // function unknown. Always set to 0 by OpenSimulator
    pub start_location: String, // The location where the user starts on login. "last", "home" or region location
    pub region_x: i64,          //The x grid coordinate of the start region in meters.
    // so a region at map co-ordinate 1000 will have a grid co-ordinate of 256000.
    pub region_y: i64,      // the y grid coordinate of the start region in meters
    pub region_size_x: i64, // the size of the start region in meters.
    // usually will be 236 but with a varregion this can be a multiple
    // of 256
    pub circuit_code: String, // Circuit code to use for all UDP connections
    pub session_id: String,   //UUID of this session
    pub secure_session_id: String, //the secure UUID of this session
    pub inventory_root: String, // the ID of the user's root folder
    pub inventory_skeleton: Vec<InvSkelValues>, // details about the child folders of the root folder.
    pub inventory_lib_root: String,             // the ID of the library root folder
    pub inventory_skel_lib: Vec<InvSkelValues>, //details about the child folders of the library root folder
    pub inventory_lib_owner: String,            // the ID of the user that owns the library
    pub map_server_url: String,                 //URL from which to request map tiles
    pub buddy_list: String, // the user's friend list. Contains an entry for each friend
}

pub struct BuddyListValues {
    pub buddy_id: String,                  //the UUID of the friend
    pub buddy_rights_given: FriendsRights, // the rights given to this user.
}

pub struct FriendsRights {
    pub uuid: String,                   //system ID of the avatar
    pub name: String,                   // full name of the avatar
    pub is_online: bool,                // true if avatar is online
    pub can_see_me_online: bool,        // true if friend can see if you are online
    pub can_see_on_map: bool,           // true if friend can see where you are on the map
    pub can_modify_my_objects: bool,    // true if friend can modify your objects
    pub can_see_them_online: bool,      // true if you can see friend online
    pub can_see_them_on_map: bool,      // true if you can see friend on map
    pub can_modify_their_objects: bool, // true if you can modify their objects
}

/// details about the child folders of the root folder
pub struct InvSkelValues {
    pub folder_id: String, // the ID of the folder
    pub parent_id: String, // the ID of the containing folder
    pub name: String,      // the name of the folder
    pub type_default: InventoryType,
    pub version: String,
}

/// enum for agent access levels
pub enum AgentAccess {
    ADULT,
    MATURE,
    PARENTALGUIDANCE,
    GENERAL,
}

/// Inventory item types
pub enum InventoryType {
    Unknown,
    Texture,
    Sound,
    CallingCard,
    Landmark,
    Object,
    Notecard,
    Category,
    Folder,
    RootCategory,
    LSL,
    Snapshot,
    Attachment,
    Wearable,
    Animation,
    Gesture,
    Mesh,
}

/// The home location of the user. In the format
/// This is in the format "{'region_handle':[r<x-grid-coord>,r<y-grid-coord>],
///     'position':[r<x-region-coord>,r<y-region-coord>,r<z-region-coord>],
///     'look_at':[r<x-coord>,r<y-coord>,r<z-coord>]} in the XML
/// For example "{'region_handle':[r256000,r256000], 'position':[r50,r100,r200], 'look_at':[r1,r0,r0]}".
pub struct HomeValues {
    pub region_handle: (i64, i64),
    pub position: (i64, i64, i64),
    pub look_at: (i64, i64, i64),
}
