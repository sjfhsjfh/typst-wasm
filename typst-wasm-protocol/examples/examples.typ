// Should compile example.wasm first
#import plugin("examples.wasm"): to_uppercase, count_chars, divide_numbers, validate_email

#let input = "Hello, Typst!"

// Call the to_uppercase function
#let uppercase = str(to_uppercase(bytes(input)))
Original: #input\
Uppercase: #uppercase

// Call the count_chars function (note this is exported with a custom name)
#let char_count = str(count_chars(bytes(input)))
#char_count

// Call a function that returns a Result type
#let division_result = str(divide_numbers(bytes("10,2")))
#division_result

// Handle potential errors, uncomment to see the error handling in action
// #let division_error = divide_numbers(bytes("10,0"))

// Use a function with Result<String, String> type
#let email_valid = str(validate_email(bytes("user@example.com")))
Email validation: #email_valid
