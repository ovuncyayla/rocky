
#[derive(Debug)]
pub struct RouteConfig {
    pub routes: Vec<RouteDef>
}

#[derive(Debug)]
pub struct RouteDef {
    path: String,
    conf: jg::Config
}

impl RouteDef {
    
}
