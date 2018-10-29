/// Enum to describe the different build tools available
pub enum BuildTool {
    MAVEN,
    GRADLE,
    ALL
}

/// Enum to describe the kind of nodes encountered in a mven settings files
/// It is voluntarily not exhaustive since we don't need all kind of nodes.
#[derive(PartialEq)]
pub enum MavenXmlConfigNodeType {
    Proxy,
    ProxyProtocol,
    ProxyHost,
    ProxyPort,
    NoProxyHosts,

    MirrorUrl,
    Mirror,
    
    Other
}