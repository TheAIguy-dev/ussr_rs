/// Define a packet.
///
/// # Example
///
/// ```
/// packet! {
///     #[derive(Debug, Clone, Copy, PartialEq, Eq)]
///     pub TestPacket {
///         #[var]
///         pub field_one: i32,
///         pub field_two: String,
///     }
///     const ID = 0x00,
///     const DIRECTION = Serverbound,
///     const STATE = Status,
///     const MIN_SIZE = i32::MIN_SIZE + String::MIN_SIZE,
///     const MAX_SIZE = i32::MAX_SIZE + String::MAX_SIZE,
/// }
///
#[macro_export]
macro_rules! packet {
    {
        $(#[$ty_meta:meta])*
        $vis:vis $name:ident {
            $(
                $(#[doc = $doc:expr])*
                $( $(@$var:tt)? #[var] )? $field_vis:vis $field_name:ident : $field_ty:ty $(= $(@$value:tt)? ($read:expr ,  $write:expr $(,)?)  )?
            ),* $(,)?
        }
        const $id_name:ident          = $id_value:literal,
        const $direction_name:ident   = $direction_value:ident,
        const $state_name:ident       = $state_value:ident,
        const $min_size_name:ident    = $min_size_value:expr,
        const $max_size_name:ident    = $max_size_value:expr $(,)?
        $(, const $can_change_state:ident = $can_change_state_value:literal $(,)?)?
    } => {
        $(#[$ty_meta])*
        pub struct $name {
            $(
                $(#[doc = $doc])*
                $field_vis $field_name : $field_ty,
            )*
        }
        impl Packet for $name {
            const $id_name: u32                     = $id_value;
            const $direction_name: PacketDirection  = $direction_value;
            const $state_name: State                = $state_value;
          $(const $can_change_state: bool           = $can_change_state_value;)?
            const $min_size_name: usize             = $min_size_value;
            const $max_size_name: usize             = $max_size_value;

            fn read(reader: &mut impl Read) -> Result<Self, PacketReadError> {
                Ok(Self {
                    $(
                        $field_name: packet!(
                            @internal { $( $($value)? + )? }
                            {
                                packet! {
                                    @internal { $( $($var)? + )? }
                                    { <$field_ty>::read_from(reader)? }
                                    { <$field_ty>::read_var_from(reader)? }
                                }
                            }
                            { $($read)?(reader)? }
                        ),
                    )*
                })
            }

            fn write(&self, writer: &mut impl Write) -> io::Result<()> {
                $(
                    packet! {
                        @internal { $( $($value)? + )? }
                        {
                            packet! {
                                @internal { $( $($var)? + )? }
                                { self.$field_name.write_to(writer)? }
                                { self.$field_name.write_var_to(writer)? }
                            }
                        }
                        { $($write)?(writer, &self.$field_name)? }
                    }
                )*
                Ok(())
            }
        }
    };
    ( @internal {   } { $($then:tt)* } { $($else:tt)* } ) => { $($then)* };
    ( @internal { + } { $($then:tt)* } { $($else:tt)* } ) => { $($else)* };
}
