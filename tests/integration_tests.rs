extern crate struct_to_string;
use struct_to_string::StructToString;

#[derive(StructToString)]
struct ComprehensiveTestStruct {
    int_field: i32,
    uint_field: u32,
    float_field: f64,
    bool_field: bool,
    char_field: char,
    str_field: String,
    option_field: Option<i32>,
    array_field: [i32; 3],
    slice_field: Vec<i32>,
    tuple_field: (i32, String),
    tuple_struct_field: TupleStruct,
    enum_field: AnEnum,
    nested_struct_field: NestedStruct,
}

struct TupleStruct(i32, f64, String);

struct NestedStruct {
    nested_field: i32,
}

enum AnEnum {
    Variant1,
    Variant2(i32),
    Variant3 { x: i32, y: i32 },
}

#[test]
fn to_rust() {
    let expected = r#"struct ComprehensiveTestStruct {
    int_field: i32,
    uint_field: u32,
    float_field: f64,
    bool_field: bool,
    char_field: char,
    str_field: String,
    option_field: Option<i32>,
    array_field: [i32;3],
    slice_field: Vec<i32>,
    tuple_field: (i32,String),
    tuple_struct_field: TupleStruct,
    enum_field: AnEnum,
    nested_struct_field: NestedStruct
}"#;

    let struct_string = ComprehensiveTestStruct::to_rust_string();

    println!("--- RUST CONVERSION --- ");
    println!("--- WHAT WAS GENERATED --- ");
    println!("{}", struct_string);
    println!("--- WHAT WAS EXPECTED --- ");
    println!("{}", expected);

    assert_eq!(struct_string, expected);
}

#[test]
fn to_typescript() {
    let expected = r#"interface ComprehensiveTestStruct {
    int_field: number;
    uint_field: number;
    float_field: number;
    bool_field: boolean;
    char_field: string;
    str_field: string;
    option_field?: number | null;
    array_field: number[];
    slice_field: number[];
    tuple_field: [number, string];
    tuple_struct_field: TupleStruct;
    enum_field: AnEnum;
    nested_struct_field: NestedStruct;
}"#;

    let struct_string = ComprehensiveTestStruct::to_typescript_string();

    println!("--- TS CONVERSION --- ");
    println!("--- WHAT WAS GENERATED --- ");
    println!("{}", struct_string);
    println!("--- WHAT WAS EXPECTED --- ");
    println!("{}", expected);

    assert_eq!(struct_string, expected);
}

#[test]
fn to_python() {
    let expected = r#"@dataclass_json
@dataclass
class ComprehensiveTestStruct:
    int_field: int
    uint_field: int
    float_field: float
    bool_field: bool
    char_field: str
    str_field: str
    option_field: Optional[int]
    array_field: List[int]
    slice_field: List[int]
    tuple_field: Tuple[int, str]
    tuple_struct_field: TupleStruct
    enum_field: AnEnum
    nested_struct_field: NestedStruct
"#;

    let struct_string = ComprehensiveTestStruct::to_python_string();

    println!("--- PYTHON CONVERSION --- ");
    println!("--- WHAT WAS GENERATED --- ");
    println!("{}", struct_string);
    println!("--- WHAT WAS EXPECTED --- ");
    println!("{}", expected);

    assert_eq!(struct_string, expected);
}

#[test]
fn to_go() {
    let expected = r#"type ComprehensiveTestStruct struct {
    int_field int32
    uint_field uint32
    float_field float64
    bool_field bool
    char_field rune
    str_field string
    option_field *int32
    array_field [3]int32
    slice_field []int32
    tuple_field struct{} // CANNOT CONVERT THIS TO THE GO PROGRAMMING LANGUAGE. TUPLES ARE UNSUPPORTED BY GO: (int32, string)
    tuple_struct_field TupleStruct
    enum_field AnEnum
    nested_struct_field NestedStruct
}"#;

    let struct_string = ComprehensiveTestStruct::to_go_string();

    println!("--- GO CONVERSION --- ");
    println!("--- WHAT WAS GENERATED --- ");
    println!("{}", struct_string);
    println!("--- WHAT WAS EXPECTED --- ");
    println!("{}", expected);

    assert_eq!(struct_string, expected);
}

#[test]
fn to_java() {
    let expected = r#"public class ComprehensiveTestStruct {
    public int int_field;
    public long uint_field;
    public double float_field;
    public boolean bool_field;
    public char char_field;
    public String str_field;
    public Integer option_field;
    public int[] array_field;
    public List<Integer> slice_field;
    public Tuple<Integer, String> tuple_field;
    public TupleStruct tuple_struct_field;
    public AnEnum enum_field;
    public NestedStruct nested_struct_field;
}"#;

    let struct_string = ComprehensiveTestStruct::to_java_string();

    println!("--- JAVA CONVERSION --- ");
    println!("--- WHAT WAS GENERATED --- ");
    println!("{}", struct_string);
    println!("--- WHAT WAS EXPECTED --- ");
    println!("{}", expected);

    assert_eq!(struct_string, expected);
}

#[test]
fn to_csharp() {
    let expected = r#"public class ComprehensiveTestStruct {
    public int int_field;
    public uint uint_field;
    public double float_field;
    public bool bool_field;
    public char char_field;
    public string str_field;
    public int? option_field;
    public int[] array_field;
    public List<int> slice_field;
    public (int, string) tuple_field;
    public TupleStruct tuple_struct_field;
    public AnEnum enum_field;
    public NestedStruct nested_struct_field;
}"#;

    let struct_string = ComprehensiveTestStruct::to_csharp_string();

    println!("--- C# CONVERSION --- ");
    println!("--- WHAT WAS GENERATED --- ");
    println!("{}", struct_string);
    println!("--- WHAT WAS EXPECTED --- ");
    println!("{}", expected);

    assert_eq!(struct_string, expected);
}
