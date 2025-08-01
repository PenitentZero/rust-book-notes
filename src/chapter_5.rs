/*
A struct, or structure, is a custom data type
that lets you name and package together
multiple related values that make up a
meaningful group.

Structs are similar to tuples, which were discussed in Chapter 3. Like tuples,
the pieces of a struct can be different types. Unlike with tuples, you’ll name
each piece of data so it’s clear what the values mean
*/

//Struct example:
#[derive(Debug)]
struct StudentInformation {
    name: String,
    #[allow(dead_code)]
    age: i32,
    grade: u8,
    #[allow(dead_code)]
    national_code: u64,
}
/*
To use a struct after we’ve defined it, we create an instance of that struct
by specifying concrete values for each of the fields. We create an instance by
stating the name of the struct and then add curly brackets containing key:
value pairs, where the keys are the names of the fields and the values are the
data we want to store in those fields.
We don’t have to specify the fields in the same order in which we declared them in the struct.
*/

fn add_student(name: String, age: i32, grade: u8, national_code: u64) -> StudentInformation {
    StudentInformation {
        name,
        age,
        grade,
        national_code,
    }
}
//The code is improved by me from the book because I thought this is better
//As you can see I didn't specify any fields, instead I just passed the given arguments into struct
//Ok later in book they did the same thing and the reason is parameters and fields has the same name!
//And we call it field init shorthand

//Using Tuple Structs Without Named Fields to Create Different Types
struct Color(i32, i32, i32);

//Unit-Like Structs Without Any Fields
/*
You can also define structs that don’t have any fields! These are called
unit-like structs because they behave similarly to (), the unit type.*/

pub fn structure() {
    let mut new_student: StudentInformation = add_student(String::from("ali"), 18, 12, 4271964367);

    //Usage of tuple struct
    let mut _new_color: Color = Color(122, 255, 0);
    println!(
        "Code color: {},{},{}",
        _new_color.0, _new_color.1, _new_color.2
    );

    //Now let's talk about struct update syntax!
    new_student.grade = 11;
    println!("{:?}", new_student);

    //we can use another struct instance for a new struct instance
    //How?
    //Like this:
    let mut second_student: StudentInformation = add_student(
        String::from("New student"),
        15,
        new_student.grade,
        3424641154,
    );
    println!(
        "the second student heap memory address of name: {:p}",
        second_student.name.as_ptr()
    );
    struct_parse(&mut second_student);
    second_student.name = String::from("New new student");

    println!(
        "the second student heap memory address of name: {:p}",
        second_student.name.as_ptr()
    );
    //As you can see here second student has different field values, but it uses the first student's grade instance
    //We can use a different method for this:\
    let third_student: StudentInformation = StudentInformation {
        name: String::from("John"),
        age: 15,
        grade: 20,
        national_code: 34256647,
    };

    //So if we try to print third student values, we get different name and age from new_student but same values as new_student
    println!("{:?}", third_student);
    /*
    To get a specific value from a struct, we can use dot notation. If we wanted
    just this user’s national code, we could use new_student.national_code wherever we wanted to
    use this value
    */
    let rectangle = (12, 12);
    println!("{}", area(rectangle));
}

//The point of crating and using this function was to see how we can give ownership to a function and after that we are no longer have access to that variable
//At this example we use second_student's name, and then we parse it into this function, and then we don't have the ownership of that
//What is the solution
//we use &string in parameters so we can access to length and capacity, but we don't own the student.name, and we can't modify it
//If we use &mut string, we can edit and modify but still we don't own the variable
fn struct_parse(student: &StudentInformation) {
    println!("struct parse: {:p}", student.name.as_ptr());
}

//calculate the dimension of a rectangle
fn area(dimension: (u32, u32)) -> u32 {
    dimension.0 * dimension.1
}
