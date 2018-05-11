use redis;

pub trait Field {
    fn to_redis_args(&self, _cmd: &mut redis::Cmd) {
        unimplemented!()
    }
}

macro_rules! ctry_field {
    (struct $name:ident {
        $($field_name:ident: $field_type:ty,)*
    }) => {
        pub struct $name {
            $($field_name: $field_type,)*
        }

        impl $name {
            pub fn new($($field_name: $field_type,)*) -> $name {
                $name {
                    $($field_name: $field_name,)*
                }
            }
        }

        impl Field for $name {
            fn to_redis_args(&self, cmd: &mut redis::Cmd) {
                $(
                    if stringify!($field_name) == "name" || stringify!($field_name) == "field_type" {
                        cmd.arg(self.$field_name.clone());
                    } else if stringify!($field_name) == "no_stem" && self.is_no_stem() {
                        cmd.arg("NOSTEM");
                    } else if stringify!($field_name) == "weight" {
                        cmd.arg("WEIGHT").arg(self.$field_name.clone());
                    } else if stringify!($field_name) == "separator" {
                        cmd.arg("SEPARATOR").arg(self.$field_name.clone());
                    } else if stringify!($field_name) == "sortable" && self.is_sortable() {
                        cmd.arg("SORTABLE");
                    } else if stringify!($field_name) == "no_index" && self.is_no_index() {
                        cmd.arg("NOINDEX");
                    }
                )*
            }
        }
    }
}


trait BooleanFieldMixin {
    fn is_no_stem(&self) -> bool {
        false
    }

    fn is_sortable(&self) -> bool {
        false
    }

    fn is_no_index(&self) -> bool {
        false
    }
}


ctry_field! {
    struct TextField {
        name: String,
        field_type: String,
        no_stem: bool,
        weight: f32,
        sortable: bool,
        no_index: bool,
    }
}

impl BooleanFieldMixin for TextField {
    fn is_no_stem(&self) -> bool {
        self.no_stem
    }

    fn is_sortable(&self) -> bool {
        self.sortable
    }

    fn is_no_index(&self) -> bool {
        self.no_index
    }
}

ctry_field! {
    struct TagField {
        name: String,
        field_type: String,
        separator: String,
        sortable: bool,
        no_index: bool,
    }
}

impl BooleanFieldMixin for TagField {
    fn is_sortable(&self) -> bool {
        self.sortable
    }

    fn is_no_index(&self) -> bool {
        self.no_index
    }
}

ctry_field! {
    struct NumericField {
        name: String,
        field_type: String,
        sortable: bool,
        no_index: bool,
    }
}

impl BooleanFieldMixin for NumericField {
    fn is_sortable(&self) -> bool {
        self.sortable
    }

    fn is_no_index(&self) -> bool {
        self.no_index
    }
}
