use std::net::ToSocketAddrs;

use ascii::AsciiString;
use prost::Message;

use crate::remote::{RemoteClient, dfproto::EmptyMessage, RpcCodes};


pub mod itemdef_instrument{
    tonic::include_proto!("itemdef_instrument");
}

pub mod remote_fortress_reader{
    tonic::include_proto!("remote_fortress_reader");
}

#[derive(Debug)]
struct IdContainer{
    get_material_list_id: i16,
    get_growth_list_id: i16,
    get_block_list_id: i16,
    check_hashes_id: i16,
    get_world_map_id: i16,
    get_region_maps_id: i16,
    get_tile_type_list_id: i16,
    get_plant_list_id: i16,
    get_unit_list_id: i16,
    get_unit_list_inside_id: i16,
    get_view_info_id: i16,
    get_map_info_id: i16,
    reset_map_hashes_id: i16,
    get_item_list_id: i16,
    get_building_def_list_id: i16,
    get_world_map_new_id: i16,
    get_region_maps_new_id: i16,
}

#[derive(Debug)]
pub struct RemoteFortressReader{
    client: RemoteClient,
    ids: IdContainer,
}

impl RemoteFortressReader{
    pub fn new<A>(url: Option<A>) -> Self
        where A: ToSocketAddrs
    {
        let mut client = match url {
            Some(url) => RemoteClient::new(url),
            None => RemoteClient::new("127.0.0.1:5000"),
        };
        let get_material_list_id        = client.bind_method("GetMaterialList", "dfproto.EmptyMessage", "RemoteFortressReader.MaterialList", Some("RemoteFortressReader"));
        let get_world_map_id            = client.bind_method("GetWorldMap", "dfproto.EmptyMessage", "RemoteFortressReader.WorldMap", Some("RemoteFortressReader"));
        let get_region_maps_id          = client.bind_method("GetRegionMaps", "dfproto.EmptyMessage", "RemoteFortressReader.RegionMaps", Some("RemoteFortressReader"));
        let get_growth_list_id          = client.bind_method("GetGrowthList", "dfproto.EmptyMessage", "RemoteFortressReader.MaterialList", Some("RemoteFortressReader"));
        let get_block_list_id           = client.bind_method("GetBlockList", "RemoteFortressReader.BlockRequest", "RemoteFortressReader.BlockList", Some("RemoteFortressReader"));
        let check_hashes_id             = client.bind_method("CheckHashes", "dfproto.EmptyMessage", "dfproto.EmptyMessage", Some("RemoteFortressReader"));
        let get_tile_type_list_id       = client.bind_method("GetTiletypeList", "dfproto.EmptyMessage", "RemoteFortressReader.TiletypeList", Some("RemoteFortressReader"));
        let get_plant_list_id           = client.bind_method("GetPlantList", "RemoteFortressReader.BlockRequest", "RemoteFortressReader.PlantList", Some("RemoteFortressReader"));
        let get_unit_list_id            = client.bind_method("GetUnitList", "dfproto.EmptyMessage", "RemoteFortressReader.UnitList", Some("RemoteFortressReader"));
        let get_unit_list_inside_id     = client.bind_method("GetUnitListInside", "RemoteFortressReader.BlockRequest", "RemoteFortressReader.UnitList", Some("RemoteFortressReader"));
        let get_view_info_id            = client.bind_method("GetViewInfo", "dfproto.EmptyMessage", "RemoteFortressReader.ViewInfo", Some("RemoteFortressReader"));
        let get_map_info_id             = client.bind_method("GetMapInfo", "dfproto.EmptyMessage", "RemoteFortressReader.MapInfo", Some("RemoteFortressReader"));
        let reset_map_hashes_id         = client.bind_method("ResetMapHashes", "dfproto.EmptyMessage", "dfproto.EmptyMessage", Some("RemoteFortressReader"));
        let get_item_list_id            = client.bind_method("GetItemList", "dfproto.EmptyMessage", "RemoteFortressReader.MaterialList", Some("RemoteFortressReader"));
        let get_building_def_list_id    = client.bind_method("GetBuildingDefList", "dfproto.EmptyMessage", "RemoteFortressReader.BuildingList", Some("RemoteFortressReader"));
        let get_world_map_new_id        = client.bind_method("GetWorldMapNew", "dfproto.EmptyMessage", "RemoteFortressReader.WorldMap", Some("RemoteFortressReader"));
        let get_region_maps_new_id       = client.bind_method("GetRegionMapsNew", "dfproto.EmptyMessage", "RemoteFortressReader.RegionMaps", Some("RemoteFortressReader"));
        Self{
            client,
            ids: IdContainer{
                get_material_list_id,
                get_growth_list_id,
                get_block_list_id,
                check_hashes_id,
                get_world_map_id,
                get_region_maps_id,
                get_plant_list_id,
                get_tile_type_list_id,
                get_unit_list_id,
                get_unit_list_inside_id,
                get_view_info_id,
                get_building_def_list_id,
                get_item_list_id,
                get_map_info_id,
                reset_map_hashes_id,
                get_world_map_new_id,
                get_region_maps_new_id,
            }
        }
    }

    pub fn get_material_list(&mut self) -> remote_fortress_reader::MaterialList{
        self.send_empty(self.ids.get_material_list_id)
    }

    pub fn get_growth_list(&mut self) -> remote_fortress_reader::MaterialList{
        self.send_empty(self.ids.get_growth_list_id)
    }

    pub fn get_world_map(&mut self) -> remote_fortress_reader::WorldMap{
        self.send_empty(self.ids.get_world_map_id)
    }

    pub fn get_region_maps(&mut self) -> remote_fortress_reader::RegionMaps{
        self.send_empty(self.ids.get_region_maps_id)
    }

    pub fn get_block_list(&mut self, block_request: remote_fortress_reader::BlockRequest) -> remote_fortress_reader::BlockList{
        self.send(self.ids.get_block_list_id,block_request)
    }

    pub fn check_hashes(&mut self){
        self.send_empty::<EmptyMessage>(self.ids.check_hashes_id);
    }

    pub fn get_tile_type_list(&mut self) -> remote_fortress_reader::TiletypeList{
        self.send_empty(self.ids.get_tile_type_list_id)
    }

    pub fn get_plant_list(&mut self, block_request: remote_fortress_reader::BlockRequest) -> remote_fortress_reader::PlantList{
        self.send(self.ids.get_plant_list_id, block_request)
    }

    pub fn get_unit_list(&mut self) -> remote_fortress_reader::UnitList{
        self.send_empty(self.ids.get_unit_list_id)
    }

    pub fn get_unit_list_inside(&mut self, block_request: remote_fortress_reader::BlockRequest) -> remote_fortress_reader::UnitList{
        self.send(self.ids.get_unit_list_inside_id, block_request)
    }

    pub fn get_view_info(&mut self) -> remote_fortress_reader::ViewInfo{
        self.send_empty(self.ids.get_view_info_id)
    }

    pub fn get_map_info(&mut self) -> remote_fortress_reader::MapInfo{
        self.send_empty(self.ids.get_map_info_id)
    }

    pub fn reset_map_hashes(&mut self){
        self.send_empty::<EmptyMessage>(self.ids.reset_map_hashes_id);
    }

    pub fn get_item_list(&mut self) -> remote_fortress_reader::MaterialList{
        self.send_empty(self.ids.get_item_list_id)
    }

    pub fn get_building_def_list(&mut self) -> remote_fortress_reader::BuildingList{
        self.send_empty(self.ids.get_building_def_list_id)
    }

    pub fn get_world_map_new(&mut self) -> remote_fortress_reader::WorldMap{
        self.send_empty(self.ids.get_world_map_new_id)
    }

    pub fn get_region_maps_new(&mut self) -> remote_fortress_reader::RegionMaps{
        self.send_empty(self.ids.get_region_maps_new_id)
    }

    fn send_empty<O>(&mut self, id: i16) -> O
        where O: Sized + Message + Default + 'static
    {
        self.send(id, EmptyMessage{})
    }

    fn send<I, O>(&mut self, id: i16, message: I) -> O
        where
            I: Sized + Message + Default + 'static,
            O: Sized + Message + Default + 'static
    {
        self.client.send_request(id, message);
        self.decode_response()
    }

    fn decode_response<T>(&mut self) -> T
        where T: Sized + Message + Default + 'static
    {
        let response = self.client.read_response();
        match response.0{
            RpcCodes::RpcReplyResult => {
                T::decode(&response.1[..]).unwrap()
            },
            RpcCodes::RpcReplyText => {
                println!("[DFHack]: {}",AsciiString::from_ascii(&response.1[..]).unwrap());
                self.decode_response()
            }
            x => panic!("Unhandled rpccode from get_world_map: {:?}",x)
        }
    }

}

