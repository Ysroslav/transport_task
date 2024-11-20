use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Coordinates {
    x: f64,
    y: f64
}

#[derive(Debug, Deserialize, Clone)]
pub struct Node {
    #[serde(rename = "id")]
    id: String,
    coordinates: Coordinates
}

impl Node {

    pub fn get_id(&self) -> String{
        self.id.clone()
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Nodes {
    node: Vec<Node>
}

impl Nodes {

    pub fn get_node_vec(&self) -> Vec<Node> {
        self.node.clone()
    }
}

#[derive(Debug, Deserialize, Clone)]
struct PreInstalledModule {
    capacity: f64,
    cost: f64
}

#[derive(Debug, Deserialize, Clone)]
pub struct AddModule {
    capacity: f64,
    cost: f64
}


#[derive(Debug, Deserialize, Clone)]
pub struct AdditionalModules {
    addModule: Vec<AddModule>
}

#[derive(Debug, Deserialize, Clone)]
pub struct Link {
    #[serde(rename = "id")]
    id: String,
    source: String,
    target: String,
    preInstalledModule: PreInstalledModule,
    additionalModules: AdditionalModules
}

impl Link {

    pub fn get_source(&self) -> String {
        self.source.clone()
    }

    pub fn get_target(&self) -> String {
        self.target.clone()
    }

    pub fn get_cost(&self) -> f64 {
        self.preInstalledModule.cost
    }

    pub fn get_capacity(&self) -> f64 {
        self.preInstalledModule.capacity
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Links {
    link: Vec<Link>
}

impl Links {

    pub fn get_vec_link(&self) -> Vec<Link> {
        self.link.clone()
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct NetworkStructure {
    nodes: Nodes,
    links: Links
}

impl NetworkStructure {

    pub fn get_links(&self) -> Links {
        self.links.clone()
    }

    pub fn get_nodes(&self) -> Nodes {
        self.nodes.clone()
    }

    pub fn get_node_count(&self) -> usize {
        self.nodes.node.len()
    }

    pub fn get_link_count(&self) -> usize {
        self.links.link.len()
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Demand {
    source: String,
    target: String,
    demandValue: f64
}

impl Demand {

    pub fn get_source(&self) -> String {
        self.source.clone()
    }

    pub fn get_target(&self) -> String {
        self.target.clone()
    }

    pub fn get_demand_vale(&self) -> f64 {
        self.demandValue
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Demands {
    demand: Vec<Demand>
}

impl Demands {

    pub fn get_demand_vec(&self) -> Vec<Demand> {
        self.demand.clone()
    }

    pub fn get_demands_count(&self) -> usize {
        self.demand.len()
    }

}

#[derive(Debug, Deserialize)]
pub struct Network {
    networkStructure: NetworkStructure,
    demands: Demands
}

impl Network {

    pub fn get_network_structure(&self) -> NetworkStructure {
        self.networkStructure.clone()
    }

    pub fn get_demands(&self) -> Demands {
        self.demands.clone()
    }
}