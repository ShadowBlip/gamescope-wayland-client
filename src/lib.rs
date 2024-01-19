// TODO: Figure out error: cannot find type `DrainMode` in this scope
//
//pub mod commit_queue {
//    use wayland_client;
//    // import objects from the core protocol if needed
//    use wayland_client::protocol::*;
//
//    // This module hosts a low-level representation of the protocol objects
//    // you will not need to interact with it yourself, but the code generated
//    // by the generate_client_code! macro will use it
//    pub mod __interfaces {
//        use wayland_client::protocol::__interfaces::*;
//        wayland_scanner::generate_interfaces!("./protocol/gamescope-commit-queue-v1.xml");
//    }
//    use self::__interfaces::*;
//
//    // This macro generates the actual types that represent the wayland objects of
//    // your custom protocol
//    //wayland_scanner::generate_client_code!("./protocol/gamescope-commit-queue-v1.xml");
//}

// Generate the bindings in their own module
pub mod control {
    use wayland_client;
    // This module hosts a low-level representation of the protocol objects
    // you will not need to interact with it yourself, but the code generated
    // by the generate_client_code! macro will use it
    pub mod __interfaces {
        wayland_scanner::generate_interfaces!("./protocol/gamescope-control.xml");
    }
    use self::__interfaces::*;

    // This macro generates the actual types that represent the wayland objects of
    // your custom protocol
    wayland_scanner::generate_client_code!("./protocol/gamescope-control.xml");
}

pub mod input_method {
    use wayland_client;
    // import objects from the core protocol if needed
    use wayland_client::protocol::*;

    // This module hosts a low-level representation of the protocol objects
    // you will not need to interact with it yourself, but the code generated
    // by the generate_client_code! macro will use it
    pub mod __interfaces {
        use wayland_client::protocol::__interfaces::*;
        wayland_scanner::generate_interfaces!("./protocol/gamescope-input-method.xml");
    }
    use self::__interfaces::*;

    // This macro generates the actual types that represent the wayland objects of
    // your custom protocol
    wayland_scanner::generate_client_code!("./protocol/gamescope-input-method.xml");
}

pub mod pipewire {
    use wayland_client;

    // This module hosts a low-level representation of the protocol objects
    // you will not need to interact with it yourself, but the code generated
    // by the generate_client_code! macro will use it
    pub mod __interfaces {
        wayland_scanner::generate_interfaces!("./protocol/gamescope-pipewire.xml");
    }
    use self::__interfaces::*;

    // This macro generates the actual types that represent the wayland objects of
    // your custom protocol
    wayland_scanner::generate_client_code!("./protocol/gamescope-pipewire.xml");
}

pub mod swapchain {
    use wayland_client;
    // import objects from the core protocol if needed
    use wayland_client::protocol::*;

    // This module hosts a low-level representation of the protocol objects
    // you will not need to interact with it yourself, but the code generated
    // by the generate_client_code! macro will use it
    pub mod __interfaces {
        use wayland_client::protocol::__interfaces::*;
        wayland_scanner::generate_interfaces!("./protocol/gamescope-swapchain.xml");
    }
    use self::__interfaces::*;

    // This macro generates the actual types that represent the wayland objects of
    // your custom protocol
    wayland_scanner::generate_client_code!("./protocol/gamescope-swapchain.xml");
}

pub mod tearing_control {
    use wayland_client;
    // import objects from the core protocol if needed
    use wayland_client::protocol::*;

    // This module hosts a low-level representation of the protocol objects
    // you will not need to interact with it yourself, but the code generated
    // by the generate_client_code! macro will use it
    pub mod __interfaces {
        use wayland_client::protocol::__interfaces::*;
        wayland_scanner::generate_interfaces!(
            "./protocol/gamescope-tearing-control-unstable-v1.xml"
        );
    }
    use self::__interfaces::*;

    // This macro generates the actual types that represent the wayland objects of
    // your custom protocol
    wayland_scanner::generate_client_code!("./protocol/gamescope-tearing-control-unstable-v1.xml");
}

pub mod xwayland {
    use wayland_client;
    // import objects from the core protocol if needed
    use wayland_client::protocol::*;

    // This module hosts a low-level representation of the protocol objects
    // you will not need to interact with it yourself, but the code generated
    // by the generate_client_code! macro will use it
    pub mod __interfaces {
        use wayland_client::protocol::__interfaces::*;
        wayland_scanner::generate_interfaces!("./protocol/gamescope-xwayland.xml");
    }
    use self::__interfaces::*;

    // This macro generates the actual types that represent the wayland objects of
    // your custom protocol
    wayland_scanner::generate_client_code!("./protocol/gamescope-xwayland.xml");
}
