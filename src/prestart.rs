struct Server {
    //message handler (handling all messages from client)
    pub client: Client,
    pub message_handler: MessageHandler,
    pub worker_pool: Vec<Simulators>,
}
pub struct Client {
    pub pos: (f64, f64),
    pub view_dims: (f64, f64),
}

pub struct MessageHandler {
    //implement later
}

pub struct Simulators {}

pub struct HexagonHelper {}
impl HexagonHelper {
    pub fn hash_to_position(hash: u32) -> (f64, f64) {
        let hash_vector: Vec<u8> = vec![];
        for n in 0..10 {
            let offset = 3 * n;
            let res: u8 = 0;
            res += (hash >> offset & 1) << 0;
            res += (hash >> (offset + 1) & 1) << 1;
            res += (hash >> (offset + 2) & 1) << 2;
            hash_vector.push(res);
        }
        Self::hash_helper(0, HexOrientation::Forward, hash_vector)
    }
    fn hash_helper(rotation: f64, orientation: HexOrientation, hash_vec: Vec<u8>) -> (f64, f64) {
        if hash_vec.is_empty() {
            return (0, 0);
        }
        let num = hash_vec.pop();
        let curr_offset = Self::calculate_hex_offset(rotation, orientation, num);
        //Calculate new position and orientation later
        let child_offset = Self::hash_helper(new_rotation, new_orientation, hash_vec);
    }
    fn calculate_hex_offset(rotation: f64, orientation: HexOrientation, num: u8) -> (f64, f64) {
        //TODO
    }
    pub fn position_to_hash(x: f64, y: f64) {}
}
enum HexOrientation {
    Forward,
    Backward,
}
