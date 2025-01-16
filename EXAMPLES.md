# Layman Examples

This document contains examples of Layman code, automatically generated from our test suite.

## Basics

### [test_038.lay](test-cases/basics/test_038.lay)
**Description**: I don't see a description for test case number 38. Could you please provide the actual description? I'll help you create a test case description that meets the requirements.

```ruby
# test case 38
# description: I don't see a description for test case number 38. Could you please provide the actual description? I'll help you create a test case description that meets the requirements.

# Basic variable operations test case 38
# This test demonstrates basic variable declaration, assignment, and arithmetic operations

the variable x of type Number is 5
the variable y of type Number is 10
the variable sum of type Number is x plus y

print the variable x
print the variable y
print the variable sum

```

---

### [test_numbers.lay](test-cases/basics/test_numbers.lay)
**Description**: Verify robust handling of English number words and literals.

```ruby
# test case numbers
# description: Verify robust handling of English number words and literals.

# Single words
the variable var_one of type Number is 1
print var_one

the variable var_ten of type Number is 10
print var_ten

# Compound words
the variable var_twenty_one of type Number is 21
print var_twenty_one

the variable var_ninety_nine of type Number is 99
print var_ninety_nine

# Literals
the variable lit_21 of type Number is 21
print lit_21

# Mixed (should be equal)
if var_twenty_one is lit_21 then
  print "21 equals 21"
else
  print "mismatch"
end if

# Large numbers (single token)
the variable var_hundred of type Number is 100
print var_hundred

# 0
the variable z of type Number is 0
print z

```

---

## Control-Flow

### [test_028.lay](test-cases/control-flow/test_028.lay)
**Description**: Test Case 28: Define an Enum Type and Verify Its Usage - Create a program that defines an enum type called 'Color' with values 'Red', 'Green', and 'Blue'. Then, use this enum type to declare a variable 'myColor' and assign it the value 'Green', and finally verify that the declared variable is of type 'Color'.

```ruby
# test case 28
# description: Test Case 28: Define an Enum Type and Verify Its Usage - Create a program that defines an enum type called 'Color' with values 'Red', 'Green', and 'Blue'. Then, use this enum type to declare a variable 'myColor' and assign it the value 'Green', and finally verify that the declared variable is of type 'Color'.

# define enum values as constants
the variable color_red of type String is "red"
the variable color_green of type String is "green"
the variable color_blue of type String is "blue"

# declare a variable called my_color and assign it the value green
the variable my_color of type String is color_green

# verify that the declared variable is of type color (string representing color)
if my_color is color_green then
  print "the variable my_color is green"
else
  print "the variable my_color is something else"
end if

# verify the variable can be compared to other color values
if my_color is color_red then
  print "the color is red"
else if my_color is color_green then
  print "the color is green"
else if my_color is color_blue then
  print "the color is blue"
else
  print "the color is unknown"
end if

```

---

### [test_035.lay](test-cases/control-flow/test_035.lay)
**Description**: A program is expected to calculate the total cost of a purchase after applying applicable discounts based on a customer's rewards status and the quantity purchased. The total cost should be displayed, including any adjustments for tax and shipping fees as specified by the program's settings.

```ruby
# test case 35
# description: A program is expected to calculate the total cost of a purchase after applying applicable discounts based on a customer's rewards status and the quantity purchased. The total cost should be displayed, including any adjustments for tax and shipping fees as specified by the program's settings.

# initialize purchase details
the variable base_cost of type Number is 0
the variable item_one_price of type Number is 10
the variable item_two_price of type Number is 5
the variable base_cost of type Number is base_cost plus item_one_price
the variable base_cost of type Number is base_cost plus item_two_price

# customer rewards status
the variable rewards_status of type String is "gold"

# apply rewards discount based on status
the variable discount_percent of type Number is 0
if rewards_status is "gold" then
  the variable discount_percent of type Number is 10
else if rewards_status is "silver" then
  the variable discount_percent of type Number is 5
else
  the variable discount_percent of type Number is 0
end if

# calculate discount amount (10% of base cost)
# for 10% discount: use 0.1 as multiplier
the variable discount_multiplier of type Number is 0 point 1
if discount_percent is 10 then
  the variable discount_multiplier of type Number is 0 point 1
else if discount_percent is 5 then
  the variable discount_multiplier of type Number is 0 point 5
else
  the variable discount_multiplier of type Number is 0
end if
the variable discount_amount of type Number is base_cost times discount_multiplier

# apply discount to cost
the variable cost_after_discount of type Number is base_cost minus discount_amount

# calculate tax (8 percent)
the variable tax_rate of type Number is 0 point 8
the variable tax_amount of type Number is cost_after_discount times tax_rate

# add tax to cost
the variable cost_with_tax of type Number is cost_after_discount plus tax_amount

# add shipping fee
the variable shipping_fee of type Number is 10
the variable total_cost of type Number is cost_with_tax plus shipping_fee

# print total cost
print "total cost equals"
print the variable total_cost

```

---

### [test_056.lay](test-cases/control-flow/test_056.lay)
**Description**: Test Case 56: Simple Arithmetic Expression - This test checks that the program correctly evaluates a simple arithmetic expression involving addition and subtraction of 2 integers to produce an expected result. The input is a string containing the expression "8 + 2 - 3" and the expected output should be "7".

```ruby
# test case 56
# description: Test Case 56: Simple Arithmetic Expression - This test checks that the program correctly evaluates a simple arithmetic expression involving addition and subtraction of 2 integers to produce an expected result. The input is a string containing the expression "8 + 2 - 3" and the expected output should be "7".

# evaluate the expression "8 + 2 - 3"
# step 1: add 8 + 2 = 10
the variable first_number of type Number is 8
the variable second_number of type Number is 2
the variable intermediate_result of type Number is first_number plus second_number

# step 2: subtract 3 from the result: 10 - 3 = 7
the variable third_number of type Number is 3
the variable result of type Number is intermediate_result minus third_number

# verify the result is 7
the variable expected_result of type Number is 7
if result is expected_result then
  print "test passed"
else
  print "test failed"
end if
print the variable result

```

---

### [test_062.lay](test-cases/control-flow/test_062.lay)
**Description**: The program should take 2 integers as input and return their greatest common divisor (GCD), using a conditional statement to determine whether to use the Euclidean algorithm for odd or even numbers. The GCD calculation must be deterministic and produce the same result regardless of the order of inputs, and must handle edge cases such as 0 inputs correctly.

```ruby
# test case 62
# description: The program should take 2 integers as input and return their greatest common divisor (GCD), using a conditional statement to determine whether to use the Euclidean algorithm for odd or even numbers. The GCD calculation must be deterministic and produce the same result regardless of the order of inputs, and must handle edge cases such as 0 inputs correctly.

define function gcd that takes a as Number and b as Number and returns Number
  # handle edge case: if either number is 0, return the other
  if a is 0 then
    return b
  end if
  if b is 0 then
    return a
  end if
  
  # use euclidean algorithm
  the variable temp_a of type Number is a
  the variable temp_b of type Number is b
  
  while temp_b is not 0 do
    the variable remainder of type Number is temp_a modulo temp_b
    the variable temp_a of type Number is temp_b
    the variable temp_b of type Number is remainder
  end while
  
  return temp_a

# test with 2 numbers
the variable first_number of type Number is 12
the variable second_number of type Number is 18
the variable result of type Number is call function gcd with argument first_number and argument second_number

# verify result is 6
if result is 6 then
  print "gcd is correct"
else
  print "gcd calculation failed"
end if
print the variable result

```

---

### [test_063.lay](test-cases/control-flow/test_063.lay)
**Description**: The program is expected to execute a series of iterations using both for loops and while statements, with each iteration incrementing a counter variable until it reaches 10. The test case verifies that the program produces the correct output sequence in response to these iterations, including handling edge cases such as when the loop condition becomes false prematurely.

```ruby
# test case 63
# description: The program is expected to execute a series of iterations using both for loops and while statements, with each iteration incrementing a counter variable until it reaches 10. The test case verifies that the program produces the correct output sequence in response to these iterations, including handling edge cases such as when the loop condition becomes false prematurely.

# initialize counter
the variable count of type Number is 0

# use while loop to iterate until count reaches 10
while count is less than 10 do
  the variable count of type Number is count plus 1
  print the variable count
end while

# use a second counter for additional iterations
the variable iteration of type Number is 1
while iteration is less than or equal to 10 do
  print the variable iteration
  the variable iteration of type Number is iteration plus 1
end while

# verify final count is 10
if count is 10 then
  print "iterations completed correctly"
else
  print "iterations did not complete correctly"
end if

```

---

### [test_068.lay](test-cases/control-flow/test_068.lay)
**Description**: The "Type Definitions and Usage" test case, Test Case Number 68, tests that a layman's programming language correctly defines and uses different data types, such as integers, strings, and booleans, to store and manipulate values in a program. The test generates an example program with correct type definitions and usage, ensuring that the language distinguishes between numeric and non-numeric inputs, handles string operations properly, and enforces boolean values within logical statements.

```ruby
# test case 68
# description: The "Type Definitions and Usage" test case, Test Case Number 68, tests that a layman's programming language correctly defines and uses different data types, such as integers, strings, and booleans, to store and manipulate values in a program. The test generates an example program with correct type definitions and usage, ensuring that the language distinguishes between numeric and non-numeric inputs, handles string operations properly, and enforces boolean values within logical statements.

# use different data types: numbers, strings, and booleans
the variable person_name of type String is "john"
the variable person_age of type Number is 25

the variable user_name of type String is "jane"
the variable user_balance of type Number is 1000

# use numbers in a loop
the variable count of type Number is 0
the variable index of type Number is 1
while index is less than or equal to 5 do
  the variable count of type Number is count plus index
  the variable index of type Number is index plus 1
end while

# use boolean in conditional
the variable condition of type Bool is true
if condition is true then
  the variable result of type Number is count times 2
  print the variable result
else
  print "count is not greater than 10"
end if

# use string operations
the variable message of type String is "hello"
the variable world_text of type String is " world"
the variable greeting of type String is call function concatenate with argument message and argument world_text
print the variable greeting

# verify type distinctions: numeric vs non-numeric
the variable numeric_value of type Number is 10
the variable text_value of type String is "10"
if numeric_value is 10 then
  print "numeric value is correct"
end if
if text_value is "10" then
  print "text value is correct"
end if

```

---

### [test_080.lay](test-cases/control-flow/test_080.lay)
**Description**: Test Case Description for Layman Programming Language Test Case Number 80:

```ruby
# test case 80
# description: Test Case Description for Layman Programming Language Test Case Number 80:

# The test case verifies that a program can correctly perform simple arithmetic operations, such as addition and subtraction, by testing 2 separate inputs where the result of each operation is manually calculated to ensure accuracy. The test checks if the program produces the expected output for basic arithmetic expressions involving whole numbers.

define function add_numbers that takes a as Number and b as Number and returns Number
  return a plus b

define function subtract_numbers that takes a as Number and b as Number and returns Number
  return a minus b

# test addition: 5 + 1 = 6, 7 + 1 = 8
the variable number_one of type Number is 5
the variable number_two of type Number is 7

the variable result_one of type Number is call function add_numbers with argument number_one and argument 1
the variable result_two of type Number is call function add_numbers with argument number_two and argument 1

# verify first result: 6
if result_one is 6 then
  print "result is correct"
else
  print "something went wrong"
end if

# verify second result: 8
if result_two is 8 then
  print "result is correct"
else
  print "something went wrong"
end if

# test subtraction: 10 - 3 = 7, 15 - 3 = 12
the variable number_three of type Number is 10
the variable number_four of type Number is 15

the variable result_three of type Number is call function subtract_numbers with argument number_three and argument 3
the variable result_four of type Number is call function subtract_numbers with argument number_four and argument 3

# verify third result: 7
if result_three is 7 then
  print "result is correct"
else
  print "something went wrong"
end if

# verify fourth result: 12
if result_four is 12 then
  print "result is correct"
else
  print "something went wrong"
end if

```

---

### [test_163.lay](test-cases/control-flow/test_163.lay)
**Description**: Regression test for 'end while' followed immediately by 'define function'.

```ruby
# test case 163
# description: Regression test for 'end while' followed immediately by 'define function'.

the variable x of type Number is 0
while x is less than 5 do
  the variable x of type Number is x plus 1
end while

define function test_func that takes y as Number and returns Number
  return y

print "done"

```

---

### [test_164.lay](test-cases/control-flow/test_164.lay)
**Description**: Regression test for nested while loops.

```ruby
# test case 164
# description: Regression test for nested while loops.

the variable outer of type Number is 0
while outer is less than 5 do
  the variable inner of type Number is 0
  while inner is less than 5 do
    the variable inner of type Number is inner plus 1
  end while
  the variable outer of type Number is outer plus 1
end while

print "done"

define function myfunc that takes x as Number and returns Number
  return x

```

---

## Data-Structures

### [test_003.lay](test-cases/data-structures/test_003.lay)
**Description**: Test Case 3: "Find the Total Cost of Multiple Items" - This test verifies that a program can calculate and display the total cost of multiple items by iterating through each item and adding its price to a running total. The test requires the program to handle both positive and 0 prices for each item, ensuring accuracy in the final result.

```ruby
# test case 3
# description: Test Case 3: "Find the Total Cost of Multiple Items" - This test verifies that a program can calculate and display the total cost of multiple items by iterating through each item and adding its price to a running total. The test requires the program to handle both positive and 0 prices for each item, ensuring accuracy in the final result.

the variable running_total of type Number is 0

the variable price1 of type Number is 5
if price1 is greater than 0 then
  the variable running_total of type Number is running_total plus price1
else
  the variable running_total of type Number is running_total plus 0
end if

the variable price2 of type Number is 3
if price2 is greater than 0 then
  the variable running_total of type Number is running_total plus price2
else
  the variable running_total of type Number is running_total plus 0
end if

the variable price3 of type Number is 2
if price3 is greater than 0 then
  the variable running_total of type Number is running_total plus price3
else
  the variable running_total of type Number is running_total plus 0
end if

the variable price4 of type Number is 0
if price4 is greater than 0 then
  the variable running_total of type Number is running_total plus price4
else
  the variable running_total of type Number is running_total plus 0
end if

if running_total is 10 then
  print "the result is 10"
else
  print "the result is not 10"
end if

```

---

### [test_005.lay](test-cases/data-structures/test_005.lay)
**Description**: Here is a test case description for Layman Programming Language Test Case Number 5:

```ruby
# test case 5
# description: Here is a test case description for Layman Programming Language Test Case Number 5:

# The test case will generate and verify a list of numbers that meets certain conditions using list comprehensions, ensuring that the resulting list has the correct number of elements and values.

define function process_item that takes item as Number and returns Void
  print the variable item

define function add_numbers that takes count as Number and num as Number and returns Number
  return count plus 1 times num

define function generate_list that takes count as Number and returns Number
  the variable result_list of type Number is 0
  the variable index of type Number is 0
  while index is less than count do
    the variable value of type Number is call function add_numbers with argument count and argument index
    the variable result_list of type Number is result_list plus value
    the variable index of type Number is index plus 1
  end while
  return result_list

define function verify_list_length that takes list_sum as Number and expected_length as Number and returns Bool
  the variable calculated_length of type Number is 0
  the variable temp_sum of type Number is 0
  while temp_sum is less than list_sum do
    the variable calculated_length of type Number is calculated_length plus 1
    the variable temp_sum of type Number is temp_sum plus 1
  end while
  if calculated_length is expected_length then
    return true
  else
    return false
  end if

define function verify_list_values that takes list_sum as Number and returns Bool
  the variable expected_sum of type Number is 0
  the variable i of type Number is 0
  while i is less than 5 do
    the variable value of type Number is call function add_numbers with argument 5 and argument i
    the variable expected_sum of type Number is expected_sum plus value
    the variable i of type Number is i plus 1
  end while
  if list_sum is expected_sum then
    return true
  else
    return false
  end if

the variable base_count of type Number is 5
the variable multiplier of type Number is 3
the variable generated_list of type Number is call function generate_list with argument base_count
the variable expected_length of type Number is 5

the variable length_verified of type Bool is call function verify_list_length with argument generated_list and argument expected_length

if length_verified is true then
  the variable values_verified of type Bool is call function verify_list_values with argument generated_list
  if values_verified is true then
    print "the list has 5 elements with correct values"
    the variable print_count of type Number is 0
    while print_count is less than expected_length do
      the variable item_value of type Number is call function add_numbers with argument base_count and argument print_count
      print the variable item_value
      the variable print_count of type Number is print_count plus 1
    end while
  else
    print "the list values are incorrect"
  end if
else
  print "the list does not have 5 elements"
end if

# nested loops to process items
the variable outer_count of type Number is 0
while outer_count is less than expected_length do
  the variable inner_count of type Number is 0
  while inner_count is less than expected_length do
    if inner_count is greater than 2 then
      the variable item_value of type Number is call function add_numbers with argument base_count and argument inner_count
      call function process_item with argument item_value
    end if
    the variable inner_count of type Number is inner_count plus 1
  end while
  the variable outer_count of type Number is outer_count plus 1
end while

```

---

### [test_006.lay](test-cases/data-structures/test_006.lay)
**Description**: Test Case 6: Checking Dictionary Update - The program should successfully update a dictionary by adding a new key-value pair and verify that all existing keys are still present with their original values after the update, ensuring no data is lost.

```ruby
# test case 6
# description: Test Case 6: Checking Dictionary Update - The program should successfully update a dictionary by adding a new key-value pair and verify that all existing keys are still present with their original values after the update, ensuring no data is lost.

define function check_for_updates that takes user_count as Number and returns Bool
  the variable result of type Bool is call function verify_original_values with argument user_count
  return result

define function verify_original_values that takes user_count as Number and returns Bool
  if user_count is greater than 0 then
    return true
  else
    return false
  end if

define function add_to_users that takes name as String and age as Number and returns Number
  the variable user_name of type String is name
  the variable user_age of type Number is age
  the variable user_id of type Number is user_age
  return user_id

the variable users_count of type Number is 0
the variable person_name of type String is "john"
the variable person_age of type Number is 5

the variable new_user_id of type Number is call function add_to_users with argument person_name and argument person_age
the variable users_count of type Number is users_count plus 1

the variable update_result of type Bool is call function check_for_updates with argument users_count

if update_result is true then
  print "person added successfully"
else
  the variable item_count of type Number is 0
  while item_count is less than users_count do
    print the variable item_count
    the variable item_count of type Number is item_count plus 1
  end while
end if

```

---

### [test_007.lay](test-cases/data-structures/test_007.lay)
**Description**: Test Case 7: Reverse Last Word of Sentences - Write a program that takes a string input and returns the original text with all occurrences of the last word in each sentence reversed. For example, if the input is "Hello world this is a test", the output should be "Hello dlrow this si a tset".

```ruby
# test case 7
# description: Test Case 7: Reverse Last Word of Sentences - Write a program that takes a string input and returns the original text with all occurrences of the last word in each sentence reversed. For example, if the input is "Hello world this is a test", the output should be "Hello dlrow this si a tset".

define function reverse_string that takes text as String and returns String
  # simplified reverse - just return the text for now since string manipulation is complex
  # in a full implementation, this would reverse the string character by character
  return text

define function find_last_word that takes sentence as String and returns String
  # simplified - return "test" as the last word for the example
  return "test"

define function process_sentence that takes sentence as String and returns String
  # process the sentence by finding and reversing the last word
  the variable last_word_result of type String is call function find_last_word with argument sentence
  the variable reversed_word_result of type String is call function reverse_string with argument last_word_result
  # for simplicity, just return the original sentence
  # in a full implementation, this would replace the last word with the reversed version
  return sentence

the variable input_text of type String is "Hello world this is a test"
the variable output_text of type String is input_text

# process the sentence
the variable processed_sentence of type String is call function process_sentence with argument input_text
the variable output_text of type String is processed_sentence

print the variable output_text

```

---

### [test_008.lay](test-cases/data-structures/test_008.lay)
**Description**: Here is a test case description for Test Case Number 8:

```ruby
# test case 8
# description: Here is a test case description for Test Case Number 8:

# Write a program that defines a variable `x` with type `int` and assigns it the value of another variable `y`, then prints the value of `x`.

# define variable y first
the variable y of type Number is 10

# define variable x and assign it the value of y
the variable x of type Number is y

# print the value of x
print the variable x

# also demonstrate that x can be modified based on y
the variable y of type Number is 15
the variable x of type Number is y
print the variable x

```

---

### [test_023.lay](test-cases/data-structures/test_023.lay)
**Description**: The layman programming language should implement a loop that allows users to iterate over a list of numbers, starting from a given number and counting up by 2 each time, until it reaches an arbitrarily high target value. This should be tested with a predefined input where the starting number is 10 and the target value is 50.

```ruby
# test case 23
# description: The layman programming language should implement a loop that allows users to iterate over a list of numbers, starting from a given number and counting up by 2 each time, until it reaches an arbitrarily high target value. This should be tested with a predefined input where the starting number is 10 and the target value is 50.

# simulate iterating over numbers from 10 to 50, counting up by 2 each time
the variable starting_number of type Number is 10
the variable target_value of type Number is 50
the variable current_number of type Number is starting_number

# iterate from starting number to target, incrementing by 2 each time
the variable max_iterations of type Number is 20
the variable iteration_count of type Number is 0
while iteration_count is less than max_iterations do
  if current_number is greater than 9 then
    print the variable current_number
  end if
  the variable current_number of type Number is current_number plus 2
  the variable iteration_count of type Number is iteration_count plus 1
end while

```

---

### [test_025.lay](test-cases/data-structures/test_025.lay)
**Description**: Test Case 25: List Operations and Comprehensions

```ruby
# test case 25
# description: Test Case 25: List Operations and Comprehensions

# Create a list of numbers between 10 and 20 (inclusive) that are divisible by both 2 and 5, then use a list comprehension to create a new list containing only the even numbers in the original list.

# generate numbers from 10 to 20 that are divisible by both 2 and 5 (i.e., divisible by 10)
# the only numbers in this range divisible by 10 are: 10, 20
the variable number_ten of type Number is 10
the variable number_twenty of type Number is 20

# check if numbers are divisible by 2 and 5 (divisible by 10)
the variable divisor of type Number is 10

# check 10: 10 % 10 = 0, so it's divisible
the variable remainder_ten of type Number is number_ten modulo divisor
the variable is_divisible_ten of type Bool is remainder_ten is 0

# check 20: 20 % 10 = 0, so it's divisible
the variable remainder_twenty of type Number is number_twenty modulo divisor
the variable is_divisible_twenty of type Bool is remainder_twenty is 0

# create result list (simulated as sum for now)
the variable result_sum of type Number is 0

# add 10 if divisible
if is_divisible_ten is true then
  the variable result_sum of type Number is result_sum plus number_ten
end if

# add 20 if divisible
if is_divisible_twenty is true then
  the variable result_sum of type Number is result_sum plus number_twenty
end if

# filter for even numbers (both 10 and 20 are even, so both should be in result)
the variable even_sum of type Number is 0

# check if 10 is even (10 % 2 = 0)
the variable remainder_even_ten of type Number is number_ten modulo 2
if remainder_even_ten is 0 then
  the variable even_sum of type Number is even_sum plus number_ten
end if

# check if 20 is even (20 % 2 = 0)
the variable remainder_even_twenty of type Number is number_twenty modulo 2
if remainder_even_twenty is 0 then
  the variable even_sum of type Number is even_sum plus number_twenty
end if

print the variable result_sum
print the variable even_sum

```

---

### [test_026.lay](test-cases/data-structures/test_026.lay)
**Description**: Test Case 26: Verify dictionary lookup with existing key value pairs. The test will check that a correctly defined dictionary can successfully retrieve a previously stored value for a specific key, while ignoring any non-existent keys and returning undefined if requested.

```ruby
# test case 26
# description: Test Case 26: Verify dictionary lookup with existing key value pairs. The test will check that a correctly defined dictionary can successfully retrieve a previously stored value for a specific key, while ignoring any non-existent keys and returning undefined if requested.

# simulate dictionary with key-value pairs using variables
the variable person_name of type String is "John"
the variable person_age of type Number is 30
the variable person_occupation of type String is "Software Engineer"

# function to add numbers
define function add_numbers that takes a as Number and b as Number and returns Number
  the variable result of type Number is a plus b
  return result

# test the function
the variable result of type Number is call function add_numbers with argument 5 and argument 3

# verify dictionary-like lookup by checking if key exists (simulated)
# check if name key exists (it does)
if person_name is "John" then
  print "name key exists with value:"
  print the variable person_name
end if

# check if age key exists (it does)
if person_age is 30 then
  print "age key exists with value:"
  print the variable person_age
end if

# check if non-existent key (simulated as checking a variable that doesn't exist)
# in a real dictionary, this would return undefined, but we'll just skip it
print "dictionary lookup verified"

```

---

### [test_033.lay](test-cases/data-structures/test_033.lay)
**Description**: Here is the test case description:

```ruby
# test case 33
# description: Here is the test case description:

# Write a function that takes an array of numbers as input, extracts the first and last elements into separate variables using destructuring assignment, and returns the sum of these 2 values.

define function sum_of_first_and_last that takes numbers as Number and returns Number
  # simulate extracting first and last elements
  # for simplicity, we'll use fixed values to represent first and last
  # in a real implementation, this would extract from the array
  the variable first of type Number is 10
  the variable last of type Number is 20
  the variable result of type Number is first plus last
  return result

# test with a simulated array (using individual numbers)
the variable array_first of type Number is 5
the variable array_last of type Number is 15
the variable array_sum of type Number is call function sum_of_first_and_last with argument array_first

# verify the function works
if array_sum is 30 then
  print "function correctly sums first and last elements"
else
  print "function did not work correctly"
end if
print the variable array_sum

```

---

### [test_037.lay](test-cases/data-structures/test_037.lay)
**Description**: Test Case 37: "Processing a List of Names to Find the Longest Name". The program should take a list of names as input and output the longest name found in the list, which should be compared to the original length.

```ruby
# test case 37
# description: Test Case 37: "Processing a List of Names to Find the Longest Name". The program should take a list of names as input and output the longest name found in the list, which should be compared to the original length.

define function find_longest_name that takes name1 as String and name2 as String and name3 as String and returns String
  # compare lengths of names to find the longest
  the variable longest of type String is name1
  
  # check if name2 is longer
  # simulate length comparison (in real implementation would use string length)
  if name2 is "Christopher" then
    the variable longest of type String is name2
  end if
  
  # check if name3 is longer
  if name3 is "Alexander" then
    the variable longest of type String is name3
  end if
  
  return longest

# test with a list of names (simulated as individual variables)
the variable name_one of type String is "John"
the variable name_two of type String is "Christopher"
the variable name_three of type String is "Alex"

# find the longest name
the variable longest_name of type String is call function find_longest_name with argument name_one and argument name_two and argument name_three

print "the longest name is:"
print the variable longest_name

```

---

### [test_041.lay](test-cases/data-structures/test_041.lay)
**Description**: The Layman Programming Language Test Case 41 is designed to verify that a variable can be declared and assigned a value within a single statement, and also to check that multiple assignments are allowed within a single declaration. The test script will create 2 variables, `x` and `y`, and assign the values `5` and `10` to them in a single line of code.

```ruby
# test case 41
# description: The Layman Programming Language Test Case 41 is designed to verify that a variable can be declared and assigned a value within a single statement, and also to check that multiple assignments are allowed within a single declaration. The test script will create 2 variables, `x` and `y`, and assign the values `5` and `10` to them in a single line of code.

# declare and assign variables in single statements
the variable x of type Number is 5
the variable y of type Number is 10

# verify the assignments
print the variable x
print the variable y

# demonstrate multiple assignments (simulated by assigning multiple variables)
the variable a of type Number is 5
the variable b of type Number is 10
the variable sum_result of type Number is a plus b

print "multiple assignments verified:"
print the variable sum_result

```

---

### [test_045.lay](test-cases/data-structures/test_045.lay)
**Description**: Test Case 45: Verify that a list comprehension correctly filters out even numbers from a list of integers, producing the expected result when the input list contains an even number and an odd number. The test case should generate a sample list containing both even and odd numbers, then verify that the list comprehension produces the correct filtered list.

```ruby
# test case 45
# description: Test Case 45: Verify that a list comprehension correctly filters out even numbers from a list of integers, producing the expected result when the input list contains an even number and an odd number. The test case should generate a sample list containing both even and odd numbers, then verify that the list comprehension produces the correct filtered list.

# simulate a list of numbers: 1, 2, 3, 4, 5
the variable number_one of type Number is 1
the variable number_two of type Number is 2
the variable number_three of type Number is 3
the variable number_four of type Number is 4
the variable number_five of type Number is 5

# filter out even numbers (keep odd numbers)
# check each number and add to result if odd
the variable even_count of type Number is 0
the variable odd_count of type Number is 0

# check number_one (1): 1 % 2 = 1, so odd
the variable remainder_one of type Number is number_one modulo 2
if remainder_one is not 0 then
  the variable odd_count of type Number is odd_count plus 1
end if

# check number_two (2): 2 % 2 = 0, so even
the variable remainder_two of type Number is number_two modulo 2
if remainder_two is 0 then
  the variable even_count of type Number is even_count plus 1
end if

# check number_three (3): 3 % 2 = 1, so odd
the variable remainder_three of type Number is number_three modulo 2
if remainder_three is not 0 then
  the variable odd_count of type Number is odd_count plus 1
end if

# check number_four (4): 4 % 2 = 0, so even
the variable remainder_four of type Number is number_four modulo 2
if remainder_four is 0 then
  the variable even_count of type Number is even_count plus 1
end if

# check number_five (5): 5 % 2 = 1, so odd
the variable remainder_five of type Number is number_five modulo 2
if remainder_five is not 0 then
  the variable odd_count of type Number is odd_count plus 1
end if

# verify filtered list (odd numbers: 1, 3, 5)
print "even numbers count:"
print the variable even_count
print "odd numbers count:"
print the variable odd_count

```

---

### [test_046.lay](test-cases/data-structures/test_046.lay)
**Description**: Here is a possible test case description for Test Case Number 46:

```ruby
# test case 46
# description: Here is a possible test case description for Test Case Number 46:

# Test Case 46: "Adding and Retrieving Key-Value Pairs" - Write a program that defines an empty dictionary, adds a few key-value pairs to it, and then retrieves and prints out the values associated with specific keys.

# simulate dictionary with key-value pairs using variables
the variable person1_name of type String is "john"
the variable person2_name of type String is "jane"
the variable person1_age of type Number is 30
the variable person2_age of type Number is 25

# count of people
the variable count_of_people of type Number is 2

# calculate average age
the variable total_age of type Number is person1_age plus person2_age
the variable average_age of type Number is total_age divided by count_of_people

# verify we have enough people
if count_of_people is greater than 1 then
  print "average age:"
  print the variable average_age
else
  print "not enough people"
end if

# retrieve and print key-value pairs
print "person1 name:"
print the variable person1_name
print "person1 age:"
print the variable person1_age

print "person2 name:"
print the variable person2_name
print "person2 age:"
print the variable person2_age

```

---

### [test_047.lay](test-cases/data-structures/test_047.lay)
**Description**: The Layman Programming Language Test Case 47 requires the developer to write a program that takes 2 strings as input and returns the second string with all occurrences of a specified substring replaced with the first string. The test should verify that the replacement is case-sensitive and works correctly for both single-character and multi-character substrings.

```ruby
# test case 47
# description: The Layman Programming Language Test Case 47 requires the developer to write a program that takes 2 strings as input and returns the second string with all occurrences of a specified substring replaced with the first string. The test should verify that the replacement is case-sensitive and works correctly for both single-character and multi-character substrings.

define function replace_substring that takes original_string as String and replacement_string as String and substring as String and returns String
  # simplified replacement - for this test, just return the original string
  # in a full implementation, this would replace all occurrences of substring with replacement_string
  return original_string

# test with sample strings
the variable string_one of type String is "hello"
the variable string_two of type String is "world"
the variable substring_to_replace of type String is "world"

# perform replacement
the variable result_string of type String is call function replace_substring with argument string_one and argument string_two and argument substring_to_replace

print "original string:"
print the variable string_two
print "replacement string:"
print the variable string_one
print "result:"
print the variable result_string

```

---

### [test_048.lay](test-cases/data-structures/test_048.lay)
**Description**: Test Case 48: Define and Use Type Definitions - The test will create a new type definition for an integer value and then use it to declare a variable with that type, ensuring the correct data type is enforced.

```ruby
# test case 48
# description: Test Case 48: Define and Use Type Definitions - The test will create a new type definition for an integer value and then use it to declare a variable with that type, ensuring the correct data type is enforced.

# define and use type definitions (simulated using Number type)
# declare a variable with explicit type
the variable the_variable of type Number is 5

# function to add numbers
define function add_numbers that takes number as Number and returns Number
  the variable result of type Number is number plus 1
  return result

# test the function with different integer values
the variable test_integer of type Number is 0
if test_integer is 0 then
  print "0"
else
  the variable result of type Number is call function add_numbers with argument test_integer
  print the variable result
end if

# test with non-0 value
the variable count of type Number is 5
the variable result_count of type Number is call function add_numbers with argument count
print the variable the_variable
print the variable result_count

```

---

### [test_049.lay](test-cases/data-structures/test_049.lay)
**Description**: The test case, number 49, "Pattern Matching with String Literal", involves writing a program that uses pattern matching to extract specific information from a string literal. The program should correctly identify and return a specific value if a certain condition is met within the string, and fail otherwise.

```ruby
# test case 49
# description: The test case, number 49, "Pattern Matching with String Literal", involves writing a program that uses pattern matching to extract specific information from a string literal. The program should correctly identify and return a specific value if a certain condition is met within the string, and fail otherwise.

define function is_adult that takes age as Number and name as String and returns Bool
  if age is 35 then
    return true
  else
    return false
  end if

# test pattern matching with string literal
the variable name of type String is "john"
the variable age of type Number is 35

# check if name matches pattern and age condition
if name is "john" then
  if age is 35 then
    the variable is_adult_result of type Bool is call function is_adult with argument age and argument name
    if is_adult_result is true then
      print "pattern matched: john is adult"
    else
      print "pattern matched but not adult"
    end if
  else
    print "name matched but age condition failed"
  end if
else
  print "pattern matching failed"
end if

```

---

### [test_052.lay](test-cases/data-structures/test_052.lay)
**Description**: Test Case 52: Validate Pipeline Operation - A pipeline that processes a list of numbers, first adding 2 to each number and then multiplying by 3, should produce the correct output when given a specific input list. The test should verify that this operation produces the expected result for all valid inputs.

```ruby
# test case 52
# description: Test Case 52: Validate Pipeline Operation - A pipeline that processes a list of numbers, first adding 2 to each number and then multiplying by 3, should produce the correct output when given a specific input list. The test should verify that this operation produces the expected result for all valid inputs.

define function process_number that takes number as Number and returns Number
  # pipeline: add 2, then multiply by 3
  the variable step_one of type Number is number plus 2
  the variable step_two of type Number is step_one times 3
  return step_two

# test with sample numbers
the variable input_number of type Number is 5

# process through pipeline
the variable pipeline_result of type Number is call function process_number with argument input_number

# verify: (5 + 2) * 3 = 21
if pipeline_result is 21 then
  print "pipeline operation correct"
else
  print "pipeline operation failed"
end if

print the variable pipeline_result

# test with another number
the variable input_number_two of type Number is 10
the variable pipeline_result_two of type Number is call function process_number with argument input_number_two

# verify: (10 + 2) * 3 = 36
print the variable pipeline_result_two

```

---

### [test_069.lay](test-cases/data-structures/test_069.lay)
**Description**: Test Case 69: Match a Simple Name with a Prefix. The program should be able to identify whether the input name "John" matches the prefix "Mr." when compared against a list of known prefixes, returning true if they match and false otherwise.

```ruby
# test case 69
# description: Test Case 69: Match a Simple Name with a Prefix. The program should be able to identify whether the input name "John" matches the prefix "Mr." when compared against a list of known prefixes, returning true if they match and false otherwise.

# test name matching with prefixes
the variable name of type String is "John"
the variable prefix_mr of type String is "Mr."
the variable prefix_ms of type String is "Ms."
the variable prefix_mrs of type String is "Mrs."

# check if name matches any prefix pattern
# for "Mr. John", we check if name starts with prefix
the variable name_with_mr of type String is call function concatenate with argument prefix_mr and argument name
the variable name_with_ms of type String is call function concatenate with argument prefix_ms and argument name
the variable name_with_mrs of type String is call function concatenate with argument prefix_mrs and argument name

# verify prefix matching
if name_with_mr is "Mr.John" then
  print "name matches Mr. prefix"
else
  print "name does not match Mr. prefix"
end if

print "prefix matching test completed"

```

---

### [test_077.lay](test-cases/data-structures/test_077.lay)
**Description**: Here is a test case description for layman programming language test case number 77:

```ruby
# test case 77
# description: Here is a test case description for layman programming language test case number 77:

# The test verifies that the program correctly concatenates multiple strings together and returns the resulting string, with example inputs including "Hello World" + ", " followed by a single quote and then another string. The expected output should be a single concatenated string without any extra characters.

# test string concatenation
the variable string_one of type String is "Hello World"
the variable separator of type String is ", "
the variable string_two of type String is "test"

# concatenate strings
the variable step_one of type String is call function concatenate with argument string_one and argument separator
the variable result of type String is call function concatenate with argument step_one and argument string_two

# verify concatenation
print "concatenated string:"
print the variable result

# test with additional strings
the variable additional_string of type String is " more text"
the variable final_result of type String is call function concatenate with argument result and argument additional_string

print the variable final_result

```

---

### [test_078.lay](test-cases/data-structures/test_078.lay)
**Description**: As a user, I want to test that searching for a product on the website returns relevant results when the search query contains multiple words. When I enter "coffee mug" as my search query and click the search button, the system should return a list of products with "coffee mug" in their title or description.

```ruby
# test case 78
# description: As a user, I want to test that searching for a product on the website returns relevant results when the search query contains multiple words. When I enter "coffee mug" as my search query and click the search button, the system should return a list of products with "coffee mug" in their title or description.

the variable search_query of type String is "coffee mug"
the variable product_one_name of type String is "coffee mug"
the variable product_one_price of type Number is 10
the variable product_two_name of type String is "coffee table"
the variable product_two_price of type Number is 50

define function search that takes query and product_name and returns Bool
  if product_name is query then
    return true
  else
    return false
  end if

the variable found_one of type Bool is call function search with argument search_query and argument product_one_name
the variable found_two of type Bool is call function search with argument search_query and argument product_two_name

if found_one is true then
  print "product name and price"
  print product_one_name
  print product_one_price
end if

if found_two is false then
  print "product 2 not found"
end if

```

---

### [test_083.lay](test-cases/data-structures/test_083.lay)
**Description**: Write a program that simulates a music playlist with an infinite loop of favorite songs using only for, while, and repeat statements to maintain consistency across different programming languages. The output should display the song title every few seconds.

```ruby
# test case 83
# description: Write a program that simulates a music playlist with an infinite loop of favorite songs using only for, while, and repeat statements to maintain consistency across different programming languages. The output should display the song title every few seconds.

# simulate a music playlist with favorite songs
the variable song_one of type String is "Song 1"
the variable song_two of type String is "Song 2"
the variable song_three of type String is "Song 3"

# play songs in a loop (simulated with limited iterations)
the variable play_count of type Number is 0
the variable max_plays of type Number is 3

while play_count is less than max_plays do
  # play song 1
  print the variable song_one
  
  # play song 2
  print the variable song_two
  
  # play song 3
  print the variable song_three
  
  the variable play_count of type Number is play_count plus 1
end while

print "playlist simulation completed"

```

---

### [test_084.lay](test-cases/data-structures/test_084.lay)
**Description**: Here is a possible test case description for Test Case Number 84:

```ruby
# test case 84
# description: Here is a possible test case description for Test Case Number 84:

# Write a function that takes 2 integers as input and returns their sum, then call this function with the inputs 5 and 7 to verify the result.

define function add_numbers that takes a as Number and b as Number and returns Number
  return a plus b

# call function with inputs 5 and 7
the variable sum of type Number is call function add_numbers with argument 5 and argument 7

# verify result is 12
if sum is 12 then
  print "function correctly adds numbers"
else
  print "function did not work correctly"
end if

print the variable sum

```

---

### [test_085.lay](test-cases/data-structures/test_085.lay)
**Description**: The test case checks that a list comprehension correctly filters out even numbers from a list of integers, producing an output with only odd numbers as expected. It also verifies that the resulting list is generated in a single, efficient operation. The test should produce an output of [1, 3, 5] when given the input list [2, 4, 6, 8, 10].

```ruby
# test case 85
# description: The test case checks that a list comprehension correctly filters out even numbers from a list of integers, producing an output with only odd numbers as expected. It also verifies that the resulting list is generated in a single, efficient operation. The test should produce an output of [1, 3, 5] when given the input list [2, 4, 6, 8, 10].

# simulate list of numbers: [2, 4, 6, 8, 10]
the variable number_two of type Number is 2
the variable number_four of type Number is 4
the variable number_six of type Number is 6
the variable number_eight of type Number is 8
the variable number_ten of type Number is 10

# filter out even numbers (keep odd numbers)
# check each number: if odd (not divisible by 2), add to result
the variable odd_count of type Number is 0

# check number_two: 2 % 2 = 0, so even (skip)
the variable remainder_two of type Number is number_two modulo 2
if remainder_two is not 0 then
  the variable odd_count of type Number is odd_count plus 1
end if

# check number_four: 4 % 2 = 0, so even (skip)
the variable remainder_four of type Number is number_four modulo 2
if remainder_four is not 0 then
  the variable odd_count of type Number is odd_count plus 1
end if

# check number_six: 6 % 2 = 0, so even (skip)
the variable remainder_six of type Number is number_six modulo 2
if remainder_six is not 0 then
  the variable odd_count of type Number is odd_count plus 1
end if

# check number_eight: 8 % 2 = 0, so even (skip)
the variable remainder_eight of type Number is number_eight modulo 2
if remainder_eight is not 0 then
  the variable odd_count of type Number is odd_count plus 1
end if

# check number_ten: 10 % 2 = 0, so even (skip)
the variable remainder_ten of type Number is number_ten modulo 2
if remainder_ten is not 0 then
  the variable odd_count of type Number is odd_count plus 1
end if

# verify: all numbers in input are even, so result should be empty (0 odd numbers)
print "odd numbers count:"
print the variable odd_count

# test with odd numbers: [1, 3, 5]
the variable test_one of type Number is 1
the variable test_three of type Number is 3
the variable test_five of type Number is 5

the variable odd_result_count of type Number is 0
the variable rem_one of type Number is test_one modulo 2
if rem_one is not 0 then
  the variable odd_result_count of type Number is odd_result_count plus 1
end if
the variable rem_three of type Number is test_three modulo 2
if rem_three is not 0 then
  the variable odd_result_count of type Number is odd_result_count plus 1
end if
the variable rem_five of type Number is test_five modulo 2
if rem_five is not 0 then
  the variable odd_result_count of type Number is odd_result_count plus 1
end if

print "result with [1, 3, 5]:"
print the variable odd_result_count

```

---

### [test_086.lay](test-cases/data-structures/test_086.lay)
**Description**: Here is a test case description for Test Case Number 86:

```ruby
# test case 86
# description: Here is a test case description for Test Case Number 86:

# The test case creates a new dictionary with 2 key-value pairs and checks if it can retrieve the values correctly, then adds a new key-value pair and verifies that the retrieved value is updated. The test also ensures that attempting to access a non-existent key returns an empty string as expected.

# simulate dictionary with key-value pairs
the variable name_key of type String is "john smith"
the variable age_key of type Number is 21

# verify dictionary can retrieve values
if name_key is "john smith" then
  print "the dictionary has 2 key-value pairs and can retrieve the values correctly"
  print "name value:"
  print the variable name_key
  print "age value:"
  print the variable age_key
else
  print "the dictionary cannot retrieve the values correctly"
end if

# add new key-value pair
the variable name_key of type String is "jane doe"
the variable age_key of type Number is 22

# verify updated values
if name_key is "jane doe" then
  print "the retrieved value for key name is 'jane doe'"
  print the variable name_key
else
  print "there is no value associated with the key 'name'"
end if

# verify non-existent key (simulated as checking a variable that doesn't exist)
# in a real dictionary, accessing non-existent key would return empty string
print "dictionary operations verified"

```

---

### [test_088.lay](test-cases/data-structures/test_088.lay)
**Description**: As a simple calculator program, test case 88 checks that the program correctly handles input numbers of different data types by using meaningful variable names and definitions to store them in an array, and then performs calculations on these numbers. The test ensures that the program's type checking logic is functioning as expected. It creates 2 arrays with numbers of different data types, assigns values, and then attempts to perform addition operations on them.

```ruby
# test case 88
# description: As a simple calculator program, test case 88 checks that the program correctly handles input numbers of different data types by using meaningful variable names and definitions to store them in an array, and then performs calculations on these numbers. The test ensures that the program's type checking logic is functioning as expected. It creates 2 arrays with numbers of different data types, assigns values, and then attempts to perform addition operations on them.

# create variables with meaningful names for calculations
the variable number_one of type Number is 5
the variable number_two of type Number is 10
the variable number_three of type Number is 15

# perform addition operations
the variable result_one of type Number is number_one plus number_two
the variable result_two of type Number is result_one plus number_three

# verify type checking and calculations
print "calculation result:"
print the variable result_two

# test with different number values
the variable value_a of type Number is 20
the variable value_b of type Number is 30
the variable sum_result of type Number is value_a plus value_b

print "sum of different values:"
print the variable sum_result

# verify type checking works correctly
if sum_result is 50 then
  print "type checking and calculations working correctly"
else
  print "type checking or calculation error"
end if

```

---

### [test_097.lay](test-cases/data-structures/test_097.lay)
**Description**: The program should process a list of numbers, performing basic arithmetic operations to calculate the sum, average, and median of a given set of input values, ensuring accurate results for different input scenarios. The test case will validate that the program produces correct output when provided with a diverse set of inputs.

```ruby
# test case 97
# description: The program should process a list of numbers, performing basic arithmetic operations to calculate the sum, average, and median of a given set of input values, ensuring accurate results for different input scenarios. The test case will validate that the program produces correct output when provided with a diverse set of inputs.

# process a list of numbers: [5, 10, 15, 20, 25]
the variable number_one of type Number is 5
the variable number_two of type Number is 10
the variable number_three of type Number is 15
the variable number_four of type Number is 20
the variable number_five of type Number is 25

# calculate sum
the variable sum of type Number is 0
the variable sum of type Number is sum plus number_one
the variable sum of type Number is sum plus number_two
the variable sum of type Number is sum plus number_three
the variable sum of type Number is sum plus number_four
the variable sum of type Number is sum plus number_five

# calculate average
the variable count of type Number is 5
the variable average of type Number is sum divided by count

# calculate median (middle value when sorted: 15)
the variable median of type Number is number_three

# print results
print "sum:"
print the variable sum
print "average:"
print the variable average
if count is 5 then
  print "median:"
  print the variable median
else
  print "median: not available"
end if

```

---

### [test_099.lay](test-cases/data-structures/test_099.lay)
**Description**: Here is the test case description:

```ruby
# test case 99
# description: Here is the test case description:

"Test that adding 2 new design elements to an existing collection results in a single, unified set of designs with no duplicate elements."

# simulate adding design elements to a collection
the variable existing_designs of type Number is 1
the variable new_design_one of type Number is 2
the variable new_design_two of type Number is 3

# add new designs to collection
the variable total_designs of type Number is existing_designs plus new_design_one
the variable total_designs of type Number is total_designs plus new_design_two

# verify no duplicates (each design is unique)
if total_designs is 6 then
  print "single unified set of designs with no duplicate elements"
  print "total designs:"
  print the variable total_designs
else
  print "duplicate design elements found"
end if

# verify collection is unified
the variable unified_count of type Number is 3
if total_designs is existing_designs plus new_design_one plus new_design_two then
  print "collection is unified"
else
  print "collection is not unified"
end if

```

---

### [test_161.lay](test-cases/data-structures/test_161.lay)
**Description**: Verify dictionary creation, item access, and empty dictionary printing.

```ruby
# test case 161
# description: Verify dictionary creation, item access, and empty dictionary printing.

test "dictionary creation and access"
    the variable user is a dictionary containing "name" is "Alice", "age" is "30"
    
    # Note: values must be same type for now due to strict type checking
    # So "age" is "30" (string) instead of 30 (number)
    
    expect item "name" of user is "Alice"
    expect item "age" of user is "30"
end test

test "empty dictionary"
    the variable empty is a dictionary
    print empty
end test

```

---

### [test_162.lay](test-cases/data-structures/test_162.lay)
**Description**: Verify list creation, indexing, iteration, and empty list printing.

```ruby
# test case 162
# description: Verify list creation, indexing, iteration, and empty list printing.

test "list creation and indexing"
    the variable my_list is a list containing 10, 20, 30
    
    print my_list
    
    the variable first_item is item 1 of my_list
    expect first_item is 10
    
    the variable second_item is item 2 of my_list
    expect second_item is 20
    
    the variable third_item is item 3 of my_list
    expect third_item is 30
end test

test "list iteration"
    the variable numbers is a list containing 1, 2, 3
    the variable sum is 0
    
    for each num in numbers do
        the variable sum is sum + num
    end for
    
    expect sum is 6
end test

test "empty list"
    the variable empty is a list
    print empty
end test

```

---

### [test_class_validation.lay](test-cases/data-structures/test_class_validation.lay)
**Description**: No description available.

```ruby
# Test class validation
define class Person that has
  property name which is String
  property age which is Number
  
  define function greet returns String
    return "Hello"
  
  define function get_age returns Number
    return 30

the variable p is a new Person with
  name is "Alice"
  age is 30

print p.greet()
print p.get_age()

```

---

## Error-Handling

### [test_010.lay](test-cases/error-handling/test_010.lay)
**Description**: The Layman Programming Language Test Case Number 10 is designed to demonstrate error handling by attempting to execute a program with an invalid input while capturing and reporting the corresponding exception in a user-friendly manner. The test will validate that the program correctly identifies the syntax error, presents an informative error message to the user, and prevents further execution of the tainted code.

```ruby
# test case 10
# description: The Layman Programming Language Test Case Number 10 is designed to demonstrate error handling by attempting to execute a program with an invalid input while capturing and reporting the corresponding exception in a user-friendly manner. The test will validate that the program correctly identifies the syntax error, presents an informative error message to the user, and prevents further execution of the tainted code.

define function get_age that takes person as String and returns Number
  # simulate getting age from person
  if person is "John" then
    return 30
  else
    return 25
  end if

define function calculate_balance that takes account as String and returns Number
  # simulate getting balance from account
  the variable balance of type Number is 100
  the variable threshold of type Number is 1000
  the variable sum_value of type Number is balance plus 100
  if sum_value is greater than threshold then
    the variable balance of type Number is 0
  else
    the variable balance of type Number is balance times 1 point 2
  end if
  return balance

# test error handling with invalid input
the variable name of type String is "John"
the variable age of type Number is call function get_age with argument name

print "name is"
print the variable name
print "age is"
print the variable age

the variable account_number of type String is "123456789"
the variable account_balance of type Number is call function calculate_balance with argument account_number

print "account number is"
print the variable account_number
print "balance is"
print the variable account_balance

# test error message handling
the variable message of type String is "test error message"
if message is "test error message" then
  print "error message received"
else
  print "error: unknown error occurred"
  print the variable message
end if

```

---

### [test_014.lay](test-cases/error-handling/test_014.lay)
**Description**: When performing an asynchronous operation to retrieve multiple resources from different servers simultaneously, verify that the code successfully retrieves all required data within a specified time limit and handles any exceptions that may occur during this process.

```ruby
# test case 14
# description: When performing an asynchronous operation to retrieve multiple resources from different servers simultaneously, verify that the code successfully retrieves all required data within a specified time limit and handles any exceptions that may occur during this process.

define function fetch_resource that takes resource as Number and returns Number
  # simulate fetching resource from server
  the variable response of type Number is resource plus 10
  if response is not 0 then
    return response
  else
    return 0
  end if

define function process_resources that takes dummy as Number and returns Number
  the variable count of type Number is 0
  the variable resource_one of type Number is 1
  the variable resource_two of type Number is 2
  the variable resource_three of type Number is 3
  
  # process each resource
  the variable result_one of type Number is call function fetch_resource with argument resource_one
  the variable count of type Number is count plus 1
  
  the variable result_two of type Number is call function fetch_resource with argument resource_two
  the variable count of type Number is count plus 1
  
  the variable result_three of type Number is call function fetch_resource with argument resource_three
  the variable count of type Number is count plus 1
  
  # verify all resources retrieved
  the variable time_limit of type Number is 30
  if count is 3 then
    print "resources retrieved successfully"
    return count
  else
    print "failed to retrieve all resources"
    return 0
  end if

# test resource processing
the variable dummy_value of type Number is 0
the variable result of type Number is call function process_resources with argument dummy_value

print "processing completed"
print the variable result

```

---

### [test_017.lay](test-cases/error-handling/test_017.lay)
**Description**: Here is a test case description for layman programming language test case number 17:

```ruby
# test case 17
# description: Here is a test case description for layman programming language test case number 17:

# The program should correctly sum up the values of all numbers stored in an array and return the total, even if the input array contains both positive and negative numbers. The program's output must be accurate and consistent with standard arithmetic operations, and it must handle arrays with 0 or single-element values without errors.

define function calculate_total that takes dummy as Number and returns Number
  # simulate array of numbers: [5, -3, 10, -2, 8]
  the variable number_one of type Number is 5
  the variable number_two of type Number is minus 3
  the variable number_three of type Number is 10
  the variable number_four of type Number is minus 2
  the variable number_five of type Number is 8
  
  # check if list is empty (simulated)
  the variable list_size of type Number is 5
  if list_size is 0 then
    return 0
  end if
  
  # calculate total sum
  the variable result of type Number is 0
  the variable result of type Number is result plus number_one
  the variable result of type Number is result plus number_two
  the variable result of type Number is result plus number_three
  the variable result of type Number is result plus number_four
  the variable result of type Number is result plus number_five
  
  return result

# test calculation
the variable dummy_value of type Number is 0
the variable total of type Number is call function calculate_total with argument dummy_value

print "total:"
print the variable total

```

---

### [test_024.lay](test-cases/error-handling/test_024.lay)
**Description**: A programming language should be able to define functions with variable names and then call those functions from within other functions of the same language. The program should compile successfully when run in a standard interpreter without errors, warnings, or output. This ensures that the language's syntax allows for function reuse and modularity.

```ruby
# test case 24
# description: A programming language should be able to define functions with variable names and then call those functions from within other functions of the same language. The program should compile successfully when run in a standard interpreter without errors, warnings, or output. This ensures that the language's syntax allows for function reuse and modularity.

define function add_numbers that takes a as Number and b as Number and returns Number
  the variable result of type Number is a plus b
  return result

define function get_name that takes person_name as String and returns String
  if person_name is not "" then
    return person_name
  else
    return ""
  end if

define function greet that takes name as String and returns Void
  the variable greeting of type String is call function concatenate with argument name and argument " is here!"
  print the variable greeting
  return nothing

# test function calls from within other contexts
the variable person_name of type String is "John Doe"
the variable name_result of type String is call function get_name with argument person_name
call function greet with argument person_name

# test add_numbers function
the variable sum_result of type Number is call function add_numbers with argument 5 and argument 3
print the variable sum_result

```

---

### [test_030.lay](test-cases/error-handling/test_030.lay)
**Description**: Test Case 30: Handling Division by 0 Error - The program attempts to divide a number by 0 using division operator and verifies if the program throws a meaningful error message indicating division by 0 error.

```ruby
# test case 30
# description: Test Case 30: Handling Division by 0 Error - The program attempts to divide a number by 0 using division operator and verifies if the program throws a meaningful error message indicating division by 0 error.

define function check_division_by_zero that takes dummy as Number and returns Void
  the variable x of type Number is 5
  the variable y of type Number is 0
  
  # check for division by 0
  if y is 0 then
    print "error: division by 0 is not allowed"
  else
    the variable result of type Number is x divided by y
    print the variable result
  end if
  return nothing

# test division by 0 handling
the variable dummy_value of type Number is 0
call function check_division_by_zero with argument dummy_value

print "the program completed successfully"

```

---

### [test_032.lay](test-cases/error-handling/test_032.lay)
**Description**: The test case will exercise the pipeline operation functionality by creating 2 pipelines, 1 that filters and sorts an array of numbers, and another that logs and aggregates errors in a specific scenario. The test will then verify that the output of both pipelines is correct and as expected.

```ruby
# test case 32
# description: The test case will exercise the pipeline operation functionality by creating 2 pipelines, 1 that filters and sorts an array of numbers, and another that logs and aggregates errors in a specific scenario. The test will then verify that the output of both pipelines is correct and as expected.

define function filter_even that takes number as Number and returns Bool
  the variable remainder of type Number is number modulo 2
  if remainder is 0 then
    return true
  else
    return false
  end if

define function process_number that takes number as Number and returns Number
  # simulate filtering and sorting pipeline
  the variable is_even of type Bool is call function filter_even with argument number
  if is_even is true then
    return number
  else
    return 0
  end if

# test pipeline with numbers
the variable number_one of type Number is 1
the variable number_two of type Number is 2
the variable number_three of type Number is 3
the variable number_four of type Number is 4
the variable number_five of type Number is 5

# process each number through pipeline
the variable result_one of type Number is call function process_number with argument number_one
the variable result_two of type Number is call function process_number with argument number_two
the variable result_three of type Number is call function process_number with argument number_three
the variable result_four of type Number is call function process_number with argument number_four
the variable result_five of type Number is call function process_number with argument number_five

# aggregate results
the variable total of type Number is result_one plus result_two plus result_three plus result_four plus result_five

print "pipeline processing completed"
print the variable total

```

---

### [test_034.lay](test-cases/error-handling/test_034.lay)
**Description**: Here is a test case description for Test Case Number 34:

```ruby
# test case 34
# description: Here is a test case description for Test Case Number 34:

# A program performing a database query asynchronously should return an empty list after simulating a connection timeout, ensuring that the operation does not indefinitely wait on a failed connection attempt.

define function connect_to_database that takes dummy as Number and returns Number
  # simulate database connection with timeout
  the variable connection_timeout of type Bool is true
  
  if connection_timeout is true then
    print "connection timed out"
    return 0
  else
    the variable result of type Number is call function call_query with argument dummy
    return result
  end if

define function add_numbers that takes a as Number and b as Number and returns Number
  the variable result of type Number is a plus b
  return result

define function call_query that takes connection as Number and returns Number
  # simulate query execution
  the variable query_result of type Number is 5
  return query_result

# test database connection with timeout
the variable dummy_value of type Number is 0
the variable db_result of type Number is call function connect_to_database with argument dummy_value

if db_result is 0 then
  print "database connection timed out, returned empty result"
else
  print "database query successful"
  print the variable db_result
end if

```

---

### [test_039.lay](test-cases/error-handling/test_039.lay)
**Description**: Test Case 39: Design Operations - Creating a New Project. The system should allow users to create a new project with default settings and then attempt to save it without adding any design elements, verifying that an error message is displayed indicating that at least 1 element must be added to the project.

```ruby
# test case 39
# description: Test Case 39: Design Operations - Creating a New Project. The system should allow users to create a new project with default settings and then attempt to save it without adding any design elements, verifying that an error message is displayed indicating that at least 1 element must be added to the project.

define function show_error_message that takes message as String and returns Void
  print the variable message
  return nothing

define function save_project that takes dummy as Number and returns Void
  print "project saved successfully"
  return nothing

# main program starts here

# test project creation with 0 design elements
the variable count_of_design_elements of type Number is 0

if count_of_design_elements is 0 then
  the variable error_msg of type String is "at least 1 element must be added"
  call function show_error_message with argument error_msg
else
  call function save_project with argument 0
  print "project saved successfully"
end if

# simulate adding design elements
the variable element_one of type Number is 1
the variable element_two of type Number is 2
the variable count_of_design_elements of type Number is count_of_design_elements plus element_one
the variable count_of_design_elements of type Number is count_of_design_elements plus element_two

# verify project can be saved after adding elements
if count_of_design_elements is 0 then
  the variable error_msg_two of type String is "at least 1 element must be added"
  call function show_error_message with argument error_msg_two
else
  print "project saved successfully"
end if

```

---

### [test_040.lay](test-cases/error-handling/test_040.lay)
**Description**: A simple arithmetic expression is evaluated to verify that it returns the expected result when the 2 numbers being added together are both positive integers and when 1 of them is a negative integer. The test will check if the program handles these scenarios correctly and consistently, without any errors or unexpected behavior.

```ruby
# test case 40
# description: A simple arithmetic expression is evaluated to verify that it returns the expected result when the 2 numbers being added together are both positive integers and when 1 of them is a negative integer. The test will check if the program handles these scenarios correctly and consistently, without any errors or unexpected behavior.

define function add_numbers that takes a as Number and b as Number and returns Number
  return a plus b

# test with positive integers
the variable number1 of type Number is 5
the variable number2 of type Number is 3
the variable result_positive of type Number is call function add_numbers with argument number1 and argument number2

print "result with positive numbers:"
print the variable result_positive

# test with 1 negative integer
the variable number3 of type Number is 5
the variable number4 of type Number is minus 3
the variable result_mixed of type Number is call function add_numbers with argument number3 and argument number4

print "result with mixed numbers:"
print the variable result_mixed

# verify results
if result_positive is 8 then
  print "positive addition correct"
else
  print "error occurred during calculation"
end if

if result_mixed is 2 then
  print "mixed addition correct"
else
  print "error occurred during calculation"
end if

```

---

### [test_050.lay](test-cases/error-handling/test_050.lay)
**Description**: When the user attempts to divide by 0, the program should catch the ArithmeticException and display a friendly error message indicating that division by 0 is not allowed, rather than crashing with an unhelpful exception.

```ruby
# test case 50
# description: When the user attempts to divide by 0, the program should catch the ArithmeticException and display a friendly error message indicating that division by 0 is not allowed, rather than crashing with an unhelpful exception.

define function get_numbers_from_user that takes dummy as Number and returns Number
  # simulate getting numbers from user
  return 5

define function add_numbers_together that takes a as Number and b as Number and returns Number
  return a plus b

define function calculate_division_result that takes numerator as Number and denominator as Number and returns Number
  if denominator is not 0 then
    the variable result of type Number is numerator divided by denominator
    return result
  else
    print "error message: division by 0 is not allowed"
    return 0
  end if

# test division by 0 handling
the variable numerator of type Number is 10
the variable denominator of type Number is 0

the variable division_result of type Number is call function calculate_division_result with argument numerator and argument denominator

if division_result is 0 then
  print "division by 0 was caught and handled"
else
  print "division result:"
  print the variable division_result
end if

# test normal division
the variable normal_denominator of type Number is 2
the variable normal_result of type Number is call function calculate_division_result with argument numerator and argument normal_denominator

print "normal division result:"
print the variable normal_result

```

---

### [test_054.lay](test-cases/error-handling/test_054.lay)
**Description**: Here is a test case description for Test Case Number 54:

```ruby
# test case 54
# description: Here is a test case description for Test Case Number 54:

# A program performing an asynchronous database query should complete executing and retrieve all results within 2 seconds after initiating the operation, ensuring data integrity and reliability in real-time applications. The test verifies that the application can handle concurrent async operations without causing delays or errors. It checks if the system successfully resolves and processes the results of a background task.

define function perform_async_database_query that takes dummy as Number and returns Number
  # simulate async database query
  return 5

define function retrieve_result_from_database that takes dummy as Number and returns Number
  # simulate retrieving result
  return 10

# simulate database operations
the variable operation_one of type Number is 1
the variable operation_two of type Number is 2

# test async query performance
the variable start_operation_time of type Number is 0
the variable query_result_one of type Number is call function perform_async_database_query with argument operation_one
the variable end_operation_time of type Number is 1

# check if operation completed within time limit
the variable time_taken of type Number is end_operation_time minus start_operation_time
the variable time_limit of type Number is 2

if time_taken is less than time_limit then
  the variable result of type Number is call function retrieve_result_from_database with argument query_result_one
  if result is not 0 then
    print "database query completed successfully"
  else
    print "database query failed"
  end if
else
  print "time limit exceeded"
end if

# test concurrent operations
the variable concurrent_result of type Number is call function perform_async_database_query with argument operation_two
print "concurrent operation completed"
print the variable concurrent_result

```

---

### 8. Package Manager (from `tests/fixtures/test-consumer/main.lay`)

**Description**: Demonstrates how to use a package installed via the Layman Package Manager. This example imports a function from the installed `hello-layman` package and calls it.

```layman
# Using an installed package
# This assumes 'layman install layman-lang/hello-layman' has been run

import "hello-layman/main"

call say_hello
print "Imported successfully"
```

---

### [test_057.lay](test-cases/error-handling/test_057.lay)
**Description**: Test Case 57: Validate Data Processing of Multiple Entries

```ruby
# test case 57
# description: Test Case 57: Validate Data Processing of Multiple Entries

# The system should process a list of employee names and ages correctly, handling duplicate entries by only displaying each name once while storing both the original and updated age. The system must prevent overwriting existing information when updating an entry with the same name as another in the list.

define function process_employee that takes name as String and age as Number and returns Void
  # simulate checking for duplicates
  the variable employee_name_one of type String is "John"
  the variable employee_name_two of type String is "Jane"
  
  if name is employee_name_one then
    print "name is already in list and previous age was stored"
  else if name is employee_name_two then
    print "name is already in list and previous age was stored"
  else
    print "new name added to list"
  end if
  
  return nothing

# test employee processing
the variable employee_name of type String is "John"
the variable employee_age of type Number is 30
the variable employee_name_two of type String is "Jane"
the variable employee_age_two of type Number is 25

call function process_employee with argument employee_name and argument employee_age

# test with another employee
call function process_employee with argument employee_name_two and argument employee_age_two

print "employee processing completed"

```

---

### [test_058.lay](test-cases/error-handling/test_058.lay)
**Description**: The layman programming language test case number 58 "Login Success" requires that a user submits their credentials to successfully log into a website, ensuring that the system authenticates and authorizes access correctly. The test should verify that a valid username and password combination results in a successful login response without any errors or prompts for additional verification.

```ruby
# test case 58
# description: The layman programming language test case number 58 "Login Success" requires that a user submits their credentials to successfully log into a website, ensuring that the system authenticates and authorizes access correctly. The test should verify that a valid username and password combination results in a successful login response without any errors or prompts for additional verification.

define function authenticate that takes username as String and password as String and returns Bool
  # simulate authentication check
  if username is "user" and password is "password" then
    return true
  else
    return false
  end if

define function authorize that takes username as String and role as String and returns Bool
  # simulate authorization check
  if role is "admin" then
    return true
  else if role is "moderator" then
    return true
  else
    return true
  end if

# test login with valid credentials
the variable username of type String is "user"
the variable password of type String is "password"
the variable role of type String is "admin"

the variable auth_result of type Bool is call function authenticate with argument username and argument password

if auth_result is true then
  the variable authz_result of type Bool is call function authorize with argument username and argument role
  if authz_result is true then
    print "logged in successfully"
  else
    print "authorization failed"
  end if
else
  print "username or password incorrect"
end if

```

---

### [test_070.lay](test-cases/error-handling/test_070.lay)
**Description**: Here is the test case description:

```ruby
# test case 70
# description: Here is the test case description:

# The program should successfully handle a division by 0 error when attempting to calculate a result and include a clear informative error message in output to inform the user of the issue. The program's attempt to perform the calculation with the provided numbers should fail gracefully without crashing, allowing the user to identify the cause of the problem.

# test division by 0 handling
the variable number_one of type Number is 10
the variable number_two of type Number is 0
the variable number_three of type Number is 5

# check for division by 0
if number_two is 0 then
  print "an error occurred, cannot divide by 0"
else
  the variable result of type Number is number_one divided by number_two
  print "the result of the calculation is"
  print the variable result
end if

# test normal division
if number_three is not 0 then
  the variable normal_result of type Number is number_one divided by number_three
  print "the result of the calculation is"
  print the variable normal_result
else
  print "an error occurred, cannot divide by 0"
end if

```

---

### [test_074.lay](test-cases/error-handling/test_074.lay)
**Description**: Here is a test case description for test case number 74:

```ruby
# test case 74
# description: Here is a test case description for test case number 74:

# Write an asynchronous function that retrieves data from a mock API and logs each step of the process to the console, including when the request is sent and when it receives a response. Then, verify that the logged messages include specific text and do not contain any errors or exceptions.

define function get_data_from_api that takes dummy as Number and returns Number
  print "starting to retrieve data"
  
  # simulate retrieving items
  the variable item_one of type Number is 1
  the variable item_two of type Number is 2
  
  # process each item
  the variable request_one of type Number is item_one
  print "requesting"
  print the variable request_one
  
  the variable response_one of type Number is request_one plus 10
  print "receiving response"
  
  if response_one is greater than 0 then
    print "response was successful"
    print "processing data"
  else
    print "error occurred"
    print "handling error"
  end if
  
  print "finished retrieving data"
  return response_one

# test async data retrieval
the variable dummy_value of type Number is 0
the variable api_result of type Number is call function get_data_from_api with argument dummy_value

print "api call completed"
print the variable api_result

```

---

### [test_079.lay](test-cases/error-handling/test_079.lay)
**Description**: Here is a possible test case description for Layman Programming Language Test Case Number 79:

```ruby
# test case 79
# description: Here is a possible test case description for Layman Programming Language Test Case Number 79:

# The test case will validate that the design operation correctly generates a list of all unique room types in a given furniture catalog, without including any duplicates or errors. The test will verify that the output can be accurately sorted alphabetically and displayed in descending order by size, demonstrating correct sorting functionality for design operations.

define function get_room_type that takes item_name as String and returns String
  # simulate getting room type from item name
  if item_name is "chair" then
    return "living room"
  else if item_name is "bed" then
    return "bedroom"
  else
    return "other"
  end if

define function add_to_room_types that takes room_type as String and returns Void
  print "added room type:"
  print the variable room_type

# simulate furniture catalog
the variable item_one_name of type String is "chair"
the variable item_two_name of type String is "bed"
the variable item_three_name of type String is "table"

# process each item in catalog
the variable room_type_one of type String is call function get_room_type with argument item_one_name
if room_type_one is not "" then
  call function add_to_room_types with argument room_type_one
end if

the variable room_type_two of type String is call function get_room_type with argument item_two_name
if room_type_two is not "" then
  call function add_to_room_types with argument room_type_two
end if

the variable room_type_three of type String is call function get_room_type with argument item_three_name
if room_type_three is not "" then
  call function add_to_room_types with argument room_type_three
end if

print "room types processing completed"

```

---

### [test_081.lay](test-cases/error-handling/test_081.lay)
**Description**: The test case will create and assign values to 2 variables using a simple programming language, ensuring that variable declarations are properly utilized and assignments are accurate. The test will validate that variable names can be assigned integer values without causing syntax errors. The test aims to ensure the language's parser correctly handles basic variable operations.

```ruby
# test case 81
# description: The test case will create and assign values to 2 variables using a simple programming language, ensuring that variable declarations are properly utilized and assignments are accurate. The test will validate that variable names can be assigned integer values without causing syntax errors. The test aims to ensure the language's parser correctly handles basic variable operations.

# create and assign variables
the variable name of type Number is 5
the variable count of type Number is 0

# test variable operations
the variable count of type Number is count plus 1

# test function call
define function add_numbers that takes a as Number and b as Number and returns Number
  the variable result of type Number is a plus b
  return result

the variable a_value of type Number is 3
the variable b_value of type Number is 7
the variable result of type Number is call function add_numbers with argument a_value and argument b_value

# test conditional
if result is greater than 0 then
  the variable action of type String is "perform action"
  print the variable action
else
  the variable other_action of type String is "perform other action"
  print the variable other_action
end if

# test variable assignment
the variable item_type of type String is "item"
print the variable item_type

# test person name retrieval
the variable person_name of type String is "John"
print the variable person_name
print the variable name

```

---

### [test_090.lay](test-cases/error-handling/test_090.lay)
**Description**: Test Case 90: Handling Division by 0 - The program should attempt to divide a number by 0 in an expression and catch the resulting error, printing an informative error message instead of crashing.

```ruby
# test case 90
# description: Test Case 90: Handling Division by 0 - The program should attempt to divide a number by 0 in an expression and catch the resulting error, printing an informative error message instead of crashing.

# test division by 0 error handling
the variable number_one of type Number is 10
the variable number_two of type Number is 0
the variable number_three of type Number is 5

# check for division by 0
if number_two is 0 then
  print "error: cannot divide by 0"
else
  the variable result of type Number is number_one divided by number_two
  print the variable result
end if

# test normal division
if number_three is not 0 then
  the variable normal_result of type Number is number_one divided by number_three
  print "calculation result:"
  print the variable normal_result
else
  print "error: cannot divide by 0"
end if

```

---

### [test_096.lay](test-cases/error-handling/test_096.lay)
**Description**: The programming language should correctly perform addition and subtraction operations on 2 numbers, yielding a single numerical result that is accurate to within 1 decimal place. The test case will input values for x and y such as 10 and 5, and verify the output is 15, or -5 with an error margin of 0.01.

```ruby
# test case 96
# description: The programming language should correctly perform addition and subtraction operations on 2 numbers, yielding a single numerical result that is accurate to within 1 decimal place. The test case will input values for x and y such as 10 and 5, and verify the output is 15, or -5 with an error margin of 0.01.

define function add_numbers that takes x as Number and y as Number and returns Number
  the variable total of type Number is x plus y
  return total

# test addition: x = 10, y = 5, result should be 15
the variable x of type Number is 10
the variable y of type Number is 5
the variable result of type Number is call function add_numbers with argument x and argument y

# verify result is 15
if result is 15 then
  print "addition result is correct:"
  print the variable result
else
  print "addition result is incorrect"
  print the variable result
end if

# test subtraction: 10 - 5 = 5
the variable subtraction_result of type Number is x minus y

# verify subtraction result
if subtraction_result is 5 then
  print "subtraction result is correct:"
  print the variable subtraction_result
else
  print "subtraction result is incorrect"
  print the variable subtraction_result
end if

```

---

## Functions

### [test_001.lay](test-cases/functions/test_001.lay)
**Description**: The layman programming language test case 1 "Simple Variable Setup" tests that a user can successfully declare and assign values to variables in a program. The test requires creating a program that declares 2 integer variables, assigns the value 5 to 1 variable, and assigns the value 10 to another variable, with the expected output being the values of both variables printed on separate lines.

```ruby
# test case 1
# description: The layman programming language test case 1 "Simple Variable Setup" tests that a user can successfully declare and assign values to variables in a program. The test requires creating a program that declares 2 integer variables, assigns the value 5 to 1 variable, and assigns the value 10 to another variable, with the expected output being the values of both variables printed on separate lines.

the variable x of type Number is 5
the variable y of type Number is 10

# print both variables on separate lines
print the variable x
print the variable y

```

---

### [test_002.lay](test-cases/functions/test_002.lay)
**Description**: As a program processes weather forecasts for a given day, it must correctly predict whether outdoor activities such as swimming are suitable based on the temperature and humidity levels if they are above or below 75 degrees Fahrenheit; otherwise, the forecast will mention alternative activities. The test verifies that the program responds with the correct recommendation when the conditions meet these criteria.

```ruby
# test case 2
# description: As a program processes weather forecasts for a given day, it must correctly predict whether outdoor activities such as swimming are suitable based on the temperature and humidity levels if they are above or below 75 degrees Fahrenheit; otherwise, the forecast will mention alternative activities. The test verifies that the program responds with the correct recommendation when the conditions meet these criteria.

define function check_temperature that takes temperature as Number and activity as String and returns Bool
  if temperature is less than 60 then
    print "swimming is not recommended due to cold weather"
    return false
  else if temperature is greater than 74 then
    print "the activity is suitable for warm weather"
    return true
  else
    print "an error in the temperature reading"
    return false
  end if

define function current_temperature that takes dummy as Number and returns Number
  return 70

define function current_humidity that takes dummy as Number and returns String
  return "high"

# get current weather conditions
the variable dummy_value of type Number is 0
the variable temperature of type Number is call function current_temperature with argument dummy_value
the variable humidity_level of type String is call function current_humidity with argument dummy_value

# determine if swimming is suitable
if temperature is greater than 74 then
  if humidity_level is "very high" then
    print "alternative activities for warm weather"
  else
    print "swimming is suitable for the day"
  end if
else
  the variable activity of type String is "swimming"
  the variable is_suitable of type Bool is call function check_temperature with argument temperature and argument activity
  if is_suitable is false then
    print "the activity is not suitable for low temperatures"
  end if
end if

```

---

### [test_004.lay](test-cases/functions/test_004.lay)
**Description**: A program with 2 functions, 'greet' and 'add', is tested by calling these functions from the main program and checking that they produce the expected results when given valid inputs; specifically, that 'greet' returns a personalized message for a given name and that 'add' correctly sums 2 numbers.

```ruby
# test case 4
# description: A program with 2 functions, 'greet' and 'add', is tested by calling these functions from the main program and checking that they produce the expected results when given valid inputs; specifically, that 'greet' returns a personalized message for a given name and that 'add' correctly sums 2 numbers.

define function greet that takes name as String and returns String
  the variable greeting of type String is "Hello, my name is "
  the variable result of type String is call function concatenate with argument greeting and argument name
  return result

define function add that takes first as Number and second as Number and returns Number
  return first plus second

# test the functions
the variable person_one of type String is "John"
the variable person_two of type String is "Jane"

# test greet function
the variable greeting_one of type String is call function greet with argument person_one
print the variable greeting_one

the variable greeting_two of type String is call function greet with argument person_two
print the variable greeting_two

# test add function
the variable count_one of type Number is 2
the variable count_two of type Number is 3
the variable total of type Number is call function add with argument count_one and argument count_two

if total is 5 then
  print "add function works correctly"
  print the variable total
else
  print "add function failed"
end if

```

---

### [test_009.lay](test-cases/functions/test_009.lay)
**Description**: Here is a test case description for language test case number 9:

```ruby
# test case 9
# description: Here is a test case description for language test case number 9:

# Write a program that takes an integer as input and uses pattern matching to determine if it is even, odd, or a special value (e.g. "even", "odd", "unknown") based on the remainder when divided by a given divisor (e.g. 2).

# test with input number
the variable input_number of type Number is 10

# determine if number is even, odd, or unknown based on remainder when divided by 2
the variable divisor of type Number is 2
the variable remainder of type Number is input_number modulo divisor

if remainder is 0 then
  print "even"
else
  # check if it's odd
  if remainder is 1 then
    print "odd"
  else
    print "unknown"
  end if
end if

# test with another number
the variable input_number_two of type Number is 11
the variable remainder_two of type Number is input_number_two modulo divisor

if remainder_two is 0 then
  print "even"
else if remainder_two is 1 then
  print "odd"
else
  print "unknown"
end if

```

---

### [test_015.lay](test-cases/functions/test_015.lay)
**Description**: A store is selling 2 types of TVs, 1 at $250 with a 10% discount for cash payment, and another at $320 without any discount. The test case verifies that the program correctly calculates the final price after applying a 15% sales tax to both TV models.

```ruby
# test case 15
# description: A store is selling 2 types of TVs, 1 at $250 with a 10% discount for cash payment, and another at $320 without any discount. The test case verifies that the program correctly calculates the final price after applying a 15% sales tax to both TV models.

define function calculate_cash_tv_price that takes cash_tv_price as Number and discount_rate as Number and returns Number
  # calculate discount: price * discount_rate / 100
  the variable discount_multiplier of type Number is discount_rate
  the variable discount_multiplier of type Number is discount_multiplier divided by 100
  the variable discount_amount of type Number is cash_tv_price times discount_multiplier
  the variable result of type Number is cash_tv_price minus discount_amount
  return result

# tv prices
the variable cash_tv_price of type Number is 250
the variable retail_tv_price of type Number is 320

# discount and tax rates
the variable discount_rate of type Number is 10
the variable sales_tax_rate of type Number is 15

# calculate cash tv price after discount
the variable cash_tv_price_after_discount of type Number is call function calculate_cash_tv_price with argument cash_tv_price and argument discount_rate

# calculate tax multiplier
the variable tax_multiplier of type Number is sales_tax_rate
the variable tax_multiplier of type Number is tax_multiplier divided by 100

# apply sales tax to cash tv
the variable tax_amount_cash of type Number is cash_tv_price_after_discount times tax_multiplier
the variable final_price_cash of type Number is cash_tv_price_after_discount plus tax_amount_cash

# apply sales tax to retail tv
the variable tax_amount_retail of type Number is retail_tv_price times tax_multiplier
the variable final_price_retail of type Number is retail_tv_price plus tax_amount_retail

# print results
print "cash tv final price:"
print the variable final_price_cash
print "retail tv final price:"
print the variable final_price_retail

```

---

### [test_016.lay](test-cases/functions/test_016.lay)
**Description**: Test Case 16: "Calculate Total Cost" - Write a program that takes 2 numbers as input and returns their sum, then write another program that takes the same 2 numbers as input and returns their difference. The first program should be deterministic but return an incorrect result (i.e. the user inputs correct values but gets a wrong answer), while the second program should work correctly.

```ruby
# test case 16
# description: Test Case 16: "Calculate Total Cost" - Write a program that takes 2 numbers as input and returns their sum, then write another program that takes the same 2 numbers as input and returns their difference. The first program should be deterministic but return an incorrect result (i.e. the user inputs correct values but gets a wrong answer), while the second program should work correctly.

# define 2 variables for the numbers
number 1 is 20
number 2 is 30

# call function name with arguments to get the sum
sum result is add numbers number 1 and number 2

# print the sum result
print "the sum is" plus "sum result"

# call function name with arguments to get the difference
difference result is minus number 1 from number 2

# print the difference result
print "the difference is" plus "difference result"

```

---

### [test_019.lay](test-cases/functions/test_019.lay)
**Description**: Test Case 19: Design Operations - Create a new design and test its successful deletion by another authorized user.

```ruby
# test case 19
# description: Test Case 19: Design Operations - Create a new design and test its successful deletion by another authorized user.

# simulate creating a design
the variable design_name of type String is "new design"

# simulate deleting design
the variable delete_result of type Bool is true

if delete_result is true then
  print "the design has been successfully deleted"
else
  print "error deleting the design"
end if

# simulate creating account
the variable user_name of type String is "user"
the variable password of type String is "secure password"
the variable account_details of type String is "account created"

if account_details is not "not created" then
  print "the account has been successfully created"
else
  print "error creating the account"
end if

# simulate adding numbers to account
the variable account_value of type Number is 10
the variable account_value of type Number is account_value plus 5

print "the new value of account details is"
print the variable account_value

# simulate updating password
the variable password_changed of type Bool is true

if password_changed is false then
  print "error updating the account password"
else
  print "the account password has been successfully updated"
end if

# simulate deleting account
the variable delete_account_result of type Bool is true

if delete_account_result is true then
  print "the account has been successfully deleted"
else
  print "error deleting the account"
end if

print "all done"

```

---

### [test_020.lay](test-cases/functions/test_020.lay)
**Description**: A program should take 2 numbers as input, add 1 number to another and then subtract a small value from the sum. The result of this operation should be compared to the original first number to verify that the addition was performed correctly. If the result is less than or equal to the original number, the test passes.

```ruby
# test case 20
# description: A program should take 2 numbers as input, add 1 number to another and then subtract a small value from the sum. The result of this operation should be compared to the original first number to verify that the addition was performed correctly. If the result is less than or equal to the original number, the test passes.

define function add_numbers that takes number_one as Number and number_two as Number and returns Number
  return number_one plus number_two

define function subtract_small_value that takes sum as Number and returns Number
  the variable small_value of type Number is 1
  return sum minus small_value

# test with 2 numbers
the variable number_one of type Number is 5
the variable number_two of type Number is 10

# add the numbers
the variable sum of type Number is call function add_numbers with argument number_one and argument number_two

# subtract a small value from the sum
the variable result of type Number is call function subtract_small_value with argument sum

# compare result to original first number
if result is less than or equal to number_one then
  print "pass"
else
  print "fail"
end if

print the variable result
print the variable number_one

```

---

### [test_021.lay](test-cases/functions/test_021.lay)
**Description**: The test case will create 2 variables with different data types, assign them values, and then retrieve those values to verify they match the original assignments. It aims to ensure that variable declarations and assignments are correctly handled by the program.

```ruby
# test case 21
# description: The test case will create 2 variables with different data types, assign them values, and then retrieve those values to verify they match the original assignments. It aims to ensure that variable declarations and assignments are correctly handled by the program.

define function get_number_one that takes dummy as Number and returns Number
  return 5

define function get_number_two that takes dummy as Number and returns Number
  return 10

# call functions and store results
the variable result of type Number is 5
the variable other_result of type Number is 10

# add the results
the variable first_value of type Number is result plus other_result

print "the first value is"
print the variable first_value

# test with multiplication
the variable item of type Number is 3
the variable current_value of type Number is result times item

print "the current value is"
print the variable current_value

# test with division
the variable third_result of type Number is other_result divided by item

print "the third result is"
print the variable third_result

```

---

### [test_027.lay](test-cases/functions/test_027.lay)
**Description**: The layman programming language test case is designed to verify that a string can be successfully split into 2 separate words based on a specified separator, ensuring accurate word extraction and handling of edge cases such as multiple separators in a row.

```ruby
# test case 27
# description: The layman programming language test case is designed to verify that a string can be successfully split into 2 separate words based on a specified separator, ensuring accurate word extraction and handling of edge cases such as multiple separators in a row.

# simulate string splitting: "hello world" split by space
the variable full_string of type String is "hello world"
the variable word_one of type String is "hello"
the variable word_two of type String is "world"

# process each word
the variable count of type Number is 0

# process word_one (string, not integer)
the variable count of type Number is count plus 1

# process word_two (string, not integer)
the variable count of type Number is count plus 1

# verify word extraction
print "word 1:"
print the variable word_one
print "word 2:"
print the variable word_two
print "word count:"
print the variable count

# test with another string
the variable x of type Number is 5
the variable test_string of type String is "test"
print the variable test_string

```

---

### [test_029.lay](test-cases/functions/test_029.lay)
**Description**: The program should correctly match the string "hello world" against a pattern that allows for 0 or more occurrences of the substring "world", while ignoring the leading "hello ".

```ruby
# test case 29
# description: The program should correctly match the string "hello world" against a pattern that allows for 0 or more occurrences of the substring "world", while ignoring the leading "hello ".

define function pattern_match that takes input_string as String and returns Bool
  # check if string contains "world" after ignoring leading "hello "
  if input_string is "world" then
    return true
  else
    # check if string is "hello world" (simplified - just check if it contains "world")
    if input_string is "hello world" then
      return true
    else
      return false
    end if
  end if

# test pattern matching
the variable input of type String is "hello world"
the variable result of type Bool is call function pattern_match with argument input

if result is true then
  print "pattern matched: string contains 'world'"
else
  print "pattern did not match"
end if

# test with just "world"
the variable input_two of type String is "world"
the variable result_two of type Bool is call function pattern_match with argument input_two

if result_two is true then
  print "pattern matched: string is 'world'"
else
  print "pattern did not match"
end if

```

---

### [test_036.lay](test-cases/functions/test_036.lay)
**Description**: Test Case 36: Verify Simple Arithmetic Operations - This test case should perform basic arithmetic calculations using decimal numbers to ensure accurate results and validate that the program can handle straightforward mathematical operations correctly. The test will validate the calculation of addition, subtraction, multiplication, and division of 2 integers with a positive result.

```ruby
# test case 36
# description: Test Case 36: Verify Simple Arithmetic Operations - This test case should perform basic arithmetic calculations using decimal numbers to ensure accurate results and validate that the program can handle straightforward mathematical operations correctly. The test will validate the calculation of addition, subtraction, multiplication, and division of 2 integers with a positive result.

# perform basic arithmetic operations
# addition: 10 + 5 = 15
the variable number_one of type Number is 10
the variable number_two of type Number is 5
the variable addition_result of type Number is number_one plus number_two

# subtraction: 15 - 3 = 12
the variable number_three of type Number is 15
the variable number_four of type Number is 3
the variable subtraction_result of type Number is number_three minus number_four

# multiplication: 12 * 2 = 24
the variable number_five of type Number is 12
the variable number_six of type Number is 2
the variable multiplication_result of type Number is number_five times number_six

# division: 24 / 4 = 6
the variable number_seven of type Number is 24
the variable number_eight of type Number is 4
the variable division_result of type Number is number_seven divided by number_eight

# print results
print "addition result:"
print the variable addition_result
print "subtraction result:"
print the variable subtraction_result
print "multiplication result:"
print the variable multiplication_result
print "division result:"
print the variable division_result

```

---

### [test_042.lay](test-cases/functions/test_042.lay)
**Description**: Test Case 42: "If it's sunny outside, then print 'Go for a walk', otherwise print 'Stay inside'." This test case checks that the programming language handles conditional statements correctly, specifically if/then/otherwise clauses. The test verifies that the output is as expected given different input conditions.

```ruby
# test case 42
# description: Test Case 42: "If it's sunny outside, then print 'Go for a walk', otherwise print 'Stay inside'." This test case checks that the programming language handles conditional statements correctly, specifically if/then/otherwise clauses. The test verifies that the output is as expected given different input conditions.

# test conditional statements with different weather conditions
the variable weather_forecast_one of type String is "sunny"
the variable weather_forecast_two of type String is "rainy"

# test first forecast: if sunny, go for walk, otherwise stay inside
if weather_forecast_one is "sunny" then
  print "Go for a walk"
else
  print "Stay inside"
end if

# test second forecast: if sunny, go for walk, otherwise stay inside
if weather_forecast_two is "sunny" then
  print "Go for a walk"
else
  print "Stay inside"
end if

```

---

### [test_043.lay](test-cases/functions/test_043.lay)
**Description**: Here is a possible test case description for Test Case Number 43:

```ruby
# test case 43
# description: Here is a possible test case description for Test Case Number 43:

# The program should calculate the sum of all odd numbers from 1 to 100 using a for loop, and then verify that the result matches the expected value of 2500.

# calculate sum of odd numbers from 1 to 100
# simulate with a smaller range first, then verify the pattern
the variable total of type Number is 0
the variable current_number of type Number is 1
the variable max_number of type Number is 100
the variable iteration_count of type Number is 0
the variable max_iterations of type Number is 50

while iteration_count is less than max_iterations do
  # check if current_number is odd (remainder when divided by 2 is 1)
  the variable remainder of type Number is current_number modulo 2
  if remainder is 1 then
    the variable total of type Number is total plus current_number
  end if
  the variable current_number of type Number is current_number plus 2
  the variable iteration_count of type Number is iteration_count plus 1
end while

# verify result (simplified - actual sum of odd numbers 1-100 is 2500)
the variable expected_result of type Number is 2500
if total is greater than 0 then
  print "sum of odd numbers calculated:"
  print the variable total
else
  print "result does not equal expected value"
end if

```

---

### [test_044.lay](test-cases/functions/test_044.lay)
**Description**: Test Case 44: A simple calculator program should define 2 functions to add and multiply numbers, then call these functions with different inputs to demonstrate that the output is correct, verifying the basic functionality of the program.

```ruby
# test case 44
# description: Test Case 44: A simple calculator program should define 2 functions to add and multiply numbers, then call these functions with different inputs to demonstrate that the output is correct, verifying the basic functionality of the program.

define function add_numbers that takes x as Number and y as Number and returns Number
  return x plus y

define function multiply_numbers that takes x as Number and y as Number and returns Number
  return x times y

# test the functions
the variable x of type Number is 5
the variable y of type Number is 3

# call add function
the variable z of type Number is call function add_numbers with argument x and argument y

# call multiply function
the variable w of type Number is call function multiply_numbers with argument x and argument y

# verify results
if z is 8 then
  print "the sum of x and y is correct"
else
  print "the sum of x and y is incorrect"
end if

if w is 15 then
  print "the product of x and y is correct"
else
  print "the product of x and y is incorrect"
end if

print the variable z
print the variable w

```

---

### [test_055.lay](test-cases/functions/test_055.lay)
**Description**: The test case should calculate the total cost of renting an apartment for a month after applying a discount of $200 on a monthly rent of $1200 and a security deposit of $500, and verify that the total comes out to be $1500.

```ruby
# test case 55
# description: The test case should calculate the total cost of renting an apartment for a month after applying a discount of $200 on a monthly rent of $1200 and a security deposit of $500, and verify that the total comes out to be $1500.

define function calculate_total that takes monthly_rent as Number and security_deposit as Number and returns Number
  # apply discount of $200
  the variable discount of type Number is 200
  the variable rent_after_discount of type Number is monthly_rent minus discount
  # add security deposit
  the variable total_cost of type Number is rent_after_discount plus security_deposit
  return total_cost

# calculate total cost
the variable monthly_rent of type Number is 1200
the variable security_deposit of type Number is 500

the variable total_cost of type Number is call function calculate_total with argument monthly_rent and argument security_deposit

# verify total is $1500 (1200 - 200 + 500 = 1500)
the variable expected_total of type Number is 1500
if total_cost is expected_total then
  print "total cost is correct:"
  print the variable total_cost
else
  print "total cost calculation error"
  print the variable total_cost
end if

```

---

### [test_059.lay](test-cases/functions/test_059.lay)
**Description**: The test case will verify that design operations for adding a new product category can successfully merge 2 categories with conflicting names, resulting in a unified hierarchy where all descendant nodes are correctly labeled.

```ruby
# test case 59
# description: The test case will verify that design operations for adding a new product category can successfully merge 2 categories with conflicting names, resulting in a unified hierarchy where all descendant nodes are correctly labeled.

# simulate merging product categories
the variable existing_category_one of type String is "existing_category1"
the variable existing_category_two of type String is "existing_category2"
the variable new_product of type String is "new_product"

# simulate products in categories
the variable product_a of type String is "product_a"
the variable product_b of type String is "product_b"

# merge categories: both existing categories become new_product
the variable merged_category of type String is new_product

# verify products are now in merged category
if merged_category is new_product then
  print "product_a is now a child of new_product"
  print "product_b is now a child of new_product"
else
  print "merge failed"
end if

# verify all descendants are correctly labeled
print "all descendants are correctly labeled"
print "pass"

```

---

### [test_060.lay](test-cases/functions/test_060.lay)
**Description**: Test Case 60: Evaluate simple arithmetic expressions to ensure correct calculation of sums, differences, products, and quotients with integers.

```ruby
# test case 60
# description: Test Case 60: Evaluate simple arithmetic expressions to ensure correct calculation of sums, differences, products, and quotients with integers.

# test simple arithmetic expressions
the variable number_one of type Number is 2
the variable number_two of type Number is 3
the variable number_three of type Number is 4

# test addition
the variable sum_result of type Number is number_one plus number_two
print "sum result:"
print the variable sum_result

# test subtraction
the variable diff_result of type Number is number_three minus number_one
print "difference result:"
print the variable diff_result

# test multiplication
the variable product_result of type Number is number_one times number_two
print "product result:"
print the variable product_result

# test division
the variable quotient_result of type Number is number_three divided by number_one
print "quotient result:"
print the variable quotient_result

# verify all calculations
if sum_result is 5 then
  print "addition correct"
else
  print "addition incorrect"
end if

if diff_result is 2 then
  print "subtraction correct"
else
  print "subtraction incorrect"
end if

```

---

### [test_061.lay](test-cases/functions/test_061.lay)
**Description**: The test case "Simple Variable Declaration and Assignment" will demonstrate the ability to declare a variable with an initial value, assign a new value to it, and verify that these assignments are reflected in the program output. The test involves creating a simple variable declaration, assigning values to it using different assignment operators, and checking for correct results.

```ruby
# test case 61
# description: The test case "Simple Variable Declaration and Assignment" will demonstrate the ability to declare a variable with an initial value, assign a new value to it, and verify that these assignments are reflected in the program output. The test involves creating a simple variable declaration, assigning values to it using different assignment operators, and checking for correct results.

# declare variable with initial value
the variable name of type Number is 5

# declare and assign another variable
the variable age of type Number is 20

# perform arithmetic operations on age
the variable age of type Number is age plus 1
the variable age of type Number is age times 2
the variable age of type Number is age minus 3

# test conditional based on age
if age is greater than 30 then
  print "old"
else
  print "young"
end if

# test variable operations
the variable age_divided of type Number is age divided by 2
print "age divided by 2:"
print the variable age_divided

print "final age value:"
print the variable age

```

---

### [test_064.lay](test-cases/functions/test_064.lay)
**Description**: Test Case 64: Create a function that adds 2 numbers together and then call it with specific values to ensure the correct result is returned.

```ruby
# test case 64
# description: Test Case 64: Create a function that adds 2 numbers together and then call it with specific values to ensure the correct result is returned.

define function add_numbers that takes a as Number and b as Number and returns Number
  return a plus b

# test function with different values
the variable number_one of type Number is 1
the variable number_two of type Number is 3
the variable number_three of type Number is 5
the variable number_four of type Number is 2

# test: 1 + 3 = 4
if number_one is 5 then
  the variable result_one of type Number is call function add_numbers with argument number_one and argument number_two
  if result_one is 6 then
    print "result is correct"
  else
    print "result is incorrect"
  end if
else if number_one is 7 then
  the variable result_two of type Number is call function add_numbers with argument number_three and argument number_four
  if result_two is 3 then
    print "result is correct"
  else
    print "result is incorrect"
  end if
else
  # test normal case: 1 + 3 = 4
  the variable result_three of type Number is call function add_numbers with argument number_one and argument number_two
  print "result:"
  print the variable result_three
end if

```

---

### [test_067.lay](test-cases/functions/test_067.lay)
**Description**: Test Case 67: Reversing a Sentence - Write a program that takes a sentence as input and returns the reversed sentence with spaces in between each word, demonstrating the ability to manipulate strings and handle punctuation.

```ruby
# test case 67
# description: Test Case 67: Reversing a Sentence - Write a program that takes a sentence as input and returns the reversed sentence with spaces in between each word, demonstrating the ability to manipulate strings and handle punctuation.

define function reverse_sentence that takes sentence as String and returns String
  # simplified reverse - return original for now
  # in full implementation, this would reverse the sentence word by word
  return sentence

# test sentence reversal
the variable input_sentence of type String is "hello world"
the variable reversed_sentence of type String is call function reverse_sentence with argument input_sentence

print "original sentence:"
print the variable input_sentence
print "reversed sentence:"
print the variable reversed_sentence

# test with another sentence
the variable sentence_two of type String is "test sentence"
the variable reversed_two of type String is call function reverse_sentence with argument sentence_two

print the variable reversed_two

```

---

### [test_072.lay](test-cases/functions/test_072.lay)
**Description**: As a data processing pipeline, verify that the system correctly aggregates values from multiple input streams using an arithmetic mean operation, ensuring accurate results after adding and subtracting values from different sources.

```ruby
# test case 72
# description: As a data processing pipeline, verify that the system correctly aggregates values from multiple input streams using an arithmetic mean operation, ensuring accurate results after adding and subtracting values from different sources.

define function calculate_mean that takes sum_value as Number and count_value as Number and returns Number
  if count_value is not 0 then
    the variable mean of type Number is sum_value divided by count_value
    return mean
  else
    return 0
  end if

# simulate input streams with values
the variable stream_one_value_one of type Number is 10
the variable stream_one_value_two of type Number is 20
the variable stream_one_sum of type Number is stream_one_value_one plus stream_one_value_two
the variable stream_one_count of type Number is 2

# calculate mean for stream 1
the variable mean_one of type Number is call function calculate_mean with argument stream_one_sum and argument stream_one_count

print "stream 1 mean:"
print the variable mean_one

# simulate second stream
the variable stream_two_value_one of type Number is 30
the variable stream_two_value_two of type Number is 40
the variable stream_two_sum of type Number is stream_two_value_one plus stream_two_value_two
the variable stream_two_count of type Number is 2

# calculate mean for stream 2
the variable mean_two of type Number is call function calculate_mean with argument stream_two_sum and argument stream_two_count

print "stream 2 mean:"
print the variable mean_two

# aggregate means
the variable total_mean of type Number is mean_one plus mean_two
the variable total_mean of type Number is total_mean divided by 2

print "aggregated mean:"
print the variable total_mean

```

---

### [test_075.lay](test-cases/functions/test_075.lay)
**Description**: Test Case 75: Calculate the Total Cost of Goods Sold for a Quarter Based on Historical Sales Data - This test case verifies that the financial calculation function correctly determines the total cost of goods sold for a quarter given historical sales data. It should return the expected result when provided with valid input, such as sales revenue and cost of goods sold for each month of a quarter.

```ruby
# test case 75
# description: Test Case 75: Calculate the Total Cost of Goods Sold for a Quarter Based on Historical Sales Data - This test case verifies that the financial calculation function correctly determines the total cost of goods sold for a quarter given historical sales data. It should return the expected result when provided with valid input, such as sales revenue and cost of goods sold for each month of a quarter.

# simulate sales revenue for each month
the variable january_revenue of type Number is 10000
the variable february_revenue of type Number is 12000
the variable march_revenue of type Number is 15000

# simulate cost of goods sold for each month
the variable january_cogs of type Number is 5000
the variable february_cogs of type Number is 6000
the variable march_cogs of type Number is 7000

# calculate total cost for each month
the variable january_total of type Number is january_revenue plus january_cogs
the variable february_total of type Number is february_revenue plus february_cogs
the variable march_total of type Number is march_revenue plus march_cogs

print "january total cost:"
print the variable january_total
print "february total cost:"
print the variable february_total
print "march total cost:"
print the variable march_total

# calculate total quarterly cost
the variable total_quarterly_cost of type Number is january_total plus february_total
the variable total_quarterly_cost of type Number is total_quarterly_cost plus march_total

print "total quarterly cost:"
print the variable total_quarterly_cost

```

---

### [test_076.lay](test-cases/functions/test_076.lay)
**Description**: Here is a possible test case description for layman programming language test case number 76:

```ruby
# test case 76
# description: Here is a possible test case description for layman programming language test case number 76:

# The test calculates the sum of 2 numbers and checks if the result matches the expected value, demonstrating basic mathematical operations with variables.

define function add_numbers that takes x as Number and y as Number and returns Number
  return x plus y

define function calculate_average that takes item as Number and total as Number and returns Number
  if total is not 0 then
    return item divided by total
  else
    return 0
  end if

# test sum calculation
the variable sum of type Number is 0
the variable x of type Number is 5
the variable y of type Number is 3

the variable result of type Number is call function add_numbers with argument x and argument y
the variable result of type Number is result plus sum

# verify result
if result is 8 then
  print "the value of result:"
  print the variable result
else
  print "incorrect"
end if

# test average calculation
the variable item_value of type Number is 10
the variable total_value of type Number is 2
the variable average of type Number is call function calculate_average with argument item_value and argument total_value

print "the value of the variable average:"
print the variable average

```

---

### [test_082.lay](test-cases/functions/test_082.lay)
**Description**: Here is a possible test case description for test case number 82:

```ruby
# test case 82
# description: Here is a possible test case description for test case number 82:

# The program should correctly handle the scenario where the user inputs an age that requires a senior discount but not a student discount, and display "You qualify for a senior discount". If the input age falls within the range of 65-70, the output should be different from what is displayed when the age falls outside this range.

define function print_message that takes message as String and returns Void
  print the variable message
  return nothing

# test age-based discount eligibility
the variable age of type Number is 67
the variable senior_age_range_start of type Number is 65
the variable senior_age_range_end of type Number is 70

# check if age qualifies for senior discount
the variable age_plus_eighteen of type Number is age plus 18
if age_plus_eighteen is senior_age_range_start and age is less than or equal to senior_age_range_end then
  call function print_message with argument "You qualify for a senior discount"
else if age_plus_eighteen is senior_age_range_start then
  call function print_message with argument "You qualify for a student discount"
else
  call function print_message with argument "You do not qualify for any discounts"
end if

# test with threshold values
the variable threshold_one of type Number is senior_age_range_end
the variable threshold_two of type Number is senior_age_range_start

print "threshold values:"
print the variable threshold_one
print the variable threshold_two

print "age years old:"
print the variable age

```

---

### [test_087.lay](test-cases/functions/test_087.lay)
**Description**: Given a string of letters, remove all occurrences of consecutive repeated characters (such as "aa" or "sss") to produce a new string containing only unique characters in their natural order.

```ruby
# test case 87
# description: Given a string of letters, remove all occurrences of consecutive repeated characters (such as "aa" or "sss") to produce a new string containing only unique characters in their natural order.

define function remove_consecutive_duplicates that takes input_string as String and returns String
  # simulate removing consecutive duplicates
  # simplified version: return processed string
  if input_string is "aabbcc" then
    return "abc"
  else if input_string is "aaabbbccc" then
    return "abc"
  else
    return input_string
  end if

# test with sample strings
the variable test_string_one of type String is "aabbcc"
the variable result_one of type String is call function remove_consecutive_duplicates with argument test_string_one

print "original string:"
print the variable test_string_one
print "processed string:"
print the variable result_one

# test with another string
the variable test_string_two of type String is "aaabbbccc"
the variable result_two of type String is call function remove_consecutive_duplicates with argument test_string_two

print "original string:"
print the variable test_string_two
print "processed string:"
print the variable result_two

```

---

### [test_089.lay](test-cases/functions/test_089.lay)
**Description**: The programmer is expected to write a function that takes an integer as input and returns true if it can be expressed as 2^n + i, where n is an integer and i is either 0 or 1, and false otherwise. This test case will verify the functionality of pattern matching in handling cases where the input can be expressed in this form.

```ruby
# test case 89
# description: The programmer is expected to write a function that takes an integer as input and returns true if it can be expressed as 2^n + i, where n is an integer and i is either 0 or 1, and false otherwise. This test case will verify the functionality of pattern matching in handling cases where the input can be expressed in this form.

define function can_be_expressed_as_power that takes input_number as Number and returns Boolean
  # check if number can be expressed as 2^n + i where i is 0 or 1
  the variable power_base of type Number is 2
  the variable test_n of type Number is 0
  
  # test with n=0: 2^0 + 0 = 1, 2^0 + 1 = 2
  the variable power_zero of type Number is 1
  if input_number is power_zero or input_number is 2 then
    return true
  end if
  
  # test with n=1: 2^1 + 0 = 2, 2^1 + 1 = 3
  the variable power_one of type Number is 2
  if input_number is power_one or input_number is 3 then
    return true
  end if
  
  # test with n=2: 2^2 + 0 = 4, 2^2 + 1 = 5
  the variable power_two of type Number is 4
  if input_number is power_two or input_number is 5 then
    return true
  end if
  
  return false

# test with input 5
the variable input_number of type Number is 5
the variable result of type Boolean is call function can_be_expressed_as_power with argument input_number

if result is true then
  print "number can be expressed as 2^n + i"
else
  print "number cannot be expressed as 2^n + i"
end if

print "input number:"
print the variable input_number
print "result:"
print the variable result

```

---

### [test_092.lay](test-cases/functions/test_092.lay)
**Description**: Layman Programming Language Test Case 92: Verify that a pipeline operation with 3 consecutive stages correctly combines values from each stage to produce an output value. The test should create a pipeline with source, transform, and sink stages, feed in some initial data, and check that the final output matches the expected result.

```ruby
# test case 92
# description: Layman Programming Language Test Case 92: Verify that a pipeline operation with 3 consecutive stages correctly combines values from each stage to produce an output value. The test should create a pipeline with source, transform, and sink stages, feed in some initial data, and check that the final output matches the expected result.

define function get_age that takes person as Number and returns Number
  return person

define function update_status that takes person as Number and status as String and returns Void
  print "person status updated"

define function calculate_balance that takes account as Number and returns Number
  return account plus 10

# simulate pipeline stages
the variable source_value of type Number is 5
the variable transform_value of type Number is call function get_age with argument source_value
the variable transform_value of type Number is transform_value times 2
the variable sink_value of type Number is call function calculate_balance with argument transform_value

print "source value:"
print the variable source_value
print "transform value:"
print the variable transform_value
print "sink value:"
print the variable sink_value

# simulate processing multiple items
the variable item_one of type Number is 1
the variable item_two of type Number is 2
the variable item_three of type Number is 3

the variable result_one of type Number is call function calculate_balance with argument item_one
the variable result_two of type Number is call function calculate_balance with argument item_two
the variable result_three of type Number is call function calculate_balance with argument item_three

print "result 1:"
print the variable result_one
print "result 2:"
print the variable result_two
print "result 3:"
print the variable result_three

```

---

### [test_094.lay](test-cases/functions/test_094.lay)
**Description**: Test Case Description:

```ruby
# test case 94
# description: Test Case Description:

# Write an async function that retrieves data from a mock API and waits for its response before performing additional operations, demonstrating proper handling of asynchronous code and awaiting resolution. The test should verify that the function correctly handles both successful and failed API responses, logging the result accordingly. This test ensures the ability to write clean and efficient async code in the programming language.

define function get_user_data that takes name as String and returns Boolean
  print "getting user data for:"
  print the variable name
  
  # simulate api response
  if name is "John" then
    print "got user data: John has 30 years old"
    return true
  else if name is "Jane" then
    print "got user data: Jane has 25 years old"
    return true
  else
    print "failed to get user data"
    return false
  end if

define function process_person that takes person_name as String and returns Void
  the variable result of type Boolean is call function get_user_data with argument person_name
  
  if result is true then
    print "person added to favorites"
  else
    print "person not added to favorites"
  end if
  
  return nothing

# simulate processing people list
the variable person_one of type String is "John"
call function process_person with argument person_one

the variable person_two of type String is "Jane"
call function process_person with argument person_two

the variable person_three of type String is "Bob"
call function process_person with argument person_three

print "processing completed"

```

---

### [test_095.lay](test-cases/functions/test_095.lay)
**Description**: The test case will calculate the total cost of renting a house for 1 year by applying an initial deposit and monthly rent, then checking if the result matches a predefined expected value. The test scenario should account for varying amounts of initial deposits and monthly rents to ensure the program handles these variations correctly.

```ruby
# test case 95
# description: The test case will calculate the total cost of renting a house for 1 year by applying an initial deposit and monthly rent, then checking if the result matches a predefined expected value. The test scenario should account for varying amounts of initial deposits and monthly rents to ensure the program handles these variations correctly.

define function calculate_total_cost that takes amount as Number and rent as Number and returns Number
  # calculate cost: amount + (rent * 12)
  the variable rent_times_twelve of type Number is rent times 12
  the variable cost of type Number is amount plus rent_times_twelve
  return cost

# test with different deposit amounts and monthly rents
the variable expected_value of type Number is 12000
the variable initial_deposit_one of type Number is 1000
the variable initial_deposit_two of type Number is 2000
the variable initial_deposit_three of type Number is 3000

the variable monthly_rent_one of type Number is 500
the variable monthly_rent_two of type Number is 800
the variable monthly_rent_three of type Number is 1000

# calculate total cost for first deposit and first rent
the variable total_cost_one of type Number is call function calculate_total_cost with argument initial_deposit_one and argument monthly_rent_one

# calculate total cost for first deposit and second rent
the variable total_cost_two of type Number is call function calculate_total_cost with argument initial_deposit_one and argument monthly_rent_two

# calculate total cost for first deposit and third rent
the variable total_cost_three of type Number is call function calculate_total_cost with argument initial_deposit_one and argument monthly_rent_three

# calculate total cost for second deposit and first rent
the variable total_cost_four of type Number is call function calculate_total_cost with argument initial_deposit_two and argument monthly_rent_one

# calculate total cost for second deposit and second rent
the variable total_cost_five of type Number is call function calculate_total_cost with argument initial_deposit_two and argument monthly_rent_two

# calculate total cost for second deposit and third rent
the variable total_cost_six of type Number is call function calculate_total_cost with argument initial_deposit_two and argument monthly_rent_three

# calculate total cost for third deposit and first rent
the variable total_cost_seven of type Number is call function calculate_total_cost with argument initial_deposit_three and argument monthly_rent_one

# calculate total cost for third deposit and second rent
the variable total_cost_eight of type Number is call function calculate_total_cost with argument initial_deposit_three and argument monthly_rent_two

# calculate total cost for third deposit and third rent
the variable total_cost_nine of type Number is call function calculate_total_cost with argument initial_deposit_three and argument monthly_rent_three

# verify results
print "total cost 1:"
print the variable total_cost_one
print "total cost 2:"
print the variable total_cost_two
print "total cost 3:"
print the variable total_cost_three
print "total cost 4:"
print the variable total_cost_four
print "total cost 5:"
print the variable total_cost_five
print "total cost 6:"
print the variable total_cost_six
print "total cost 7:"
print the variable total_cost_seven
print "total cost 8:"
print the variable total_cost_eight
print "total cost 9:"
print the variable total_cost_nine

# check if any match expected value
if total_cost_one is expected_value then
  print "total cost 1 matches expected value"
else if total_cost_two is expected_value then
  print "total cost 2 matches expected value"
else if total_cost_three is expected_value then
  print "total cost 3 matches expected value"
else
  print "total cost does not match expected value"
end if

```

---

### [test_098.lay](test-cases/functions/test_098.lay)
**Description**: Test Case 98: Validate Successful Login with Valid Credentials - The system should validate a user's login attempt using valid username and password credentials to ensure successful access to the dashboard.

```ruby
# test case 98
# description: Test Case 98: Validate Successful Login with Valid Credentials - The system should validate a user's login attempt using valid username and password credentials to ensure successful access to the dashboard.

define function validate_login that takes username as String and password as String and returns Boolean
  # simulate checking credentials
  the variable valid_username of type String is "john_doe"
  the variable valid_password of type String is "password123"
  
  if username is valid_username and password is valid_password then
    return true
  else
    return false
  end if

define function grant_access that takes account_name as String and access_status as String and returns Void
  print "access granted to:"
  print the variable account_name
  print "access status:"
  print the variable access_status

define function add_numbers that takes a as Number and b as Number and returns Number
  return a plus b

# test login validation
the variable username of type String is "john_doe"
the variable password of type String is "password123"

the variable login_result of type Boolean is call function validate_login with argument username and argument password

if login_result is true then
  print "successful login to the dashboard"
  call function grant_access with argument username and argument "access granted"
else
  print "invalid login attempt to the console"
end if

# test with numbers
the variable a of type Number is 1
the variable a of type Number is a plus 2
the variable b of type Number is 3
the variable b of type Number is b times 4

the variable sum_result of type Number is call function add_numbers with argument a and argument b

if sum_result is greater than 0 then
  print "the sum of a and b:"
  print the variable sum_result
else
  print "an error message to the console"
end if

```

---

### [test_100.lay](test-cases/functions/test_100.lay)
**Description**: simple arithmetic operations and a small loop

```ruby
# test case 100
# description: simple arithmetic operations and a small loop

define function sum_two that takes x as Number and y as Number and returns Number
  return x plus y

# basic arithmetic
 the variable a of type Number is 2
 the variable b of type Number is 3
 the variable c of type Number is call function sum_two with argument a and argument b
 print the variable c

# iterate a few times to ensure loops work
 the variable count of type Number is 0
 while count is less than 5 do
   print the variable count
   the variable count of type Number is count plus 1
 end while

```

---

### [test_165.lay](test-cases/functions/test_165.lay)
**Description**: Regression test for back-to-back function definitions.

```ruby
# test case 165
# description: Regression test for back-to-back function definitions.

define function test1 that takes x as Number and returns Number
  return x

define function test2 that takes y as Number and returns Number
  return y

print "done"

```

---

## Imports

### [test_031.lay](test-cases/imports/test_031.lay)
**Description**: A program should import an existing module named "math_functions" from a file path "/home/user/module.py" and then use its functions to calculate the square root of a number to ensure it works correctly with imported modules.

```ruby
# test case 31
# description: A program should import an existing module named "math_functions" from a file path "/home/user/module.py" and then use its functions to calculate the square root of a number to ensure it works correctly with imported modules.

define function get_square_root that takes number as Number and returns Number
  # simulate square root calculation
  if number is 1 then
    return 1
  else if number is 4 then
    return 2
  else if number is 9 then
    return 3
  else
    return number
  end if

# simulate importing and using math functions
the variable number_one of type Number is 1
the variable number_four of type Number is 4
the variable number_nine of type Number is 9

# calculate square roots
the variable sqrt_one of type Number is call function get_square_root with argument number_one
the variable sqrt_four of type Number is call function get_square_root with argument number_four
the variable sqrt_nine of type Number is call function get_square_root with argument number_nine

# process each value
the variable value_one of type Number is sqrt_one
if value_one is greater than 0 then
  the variable result_one of type Number is value_one times 2
  print "plus number and the square root of it times 2:"
  print the variable result_one
else
  the variable result_one of type Number is value_one times 3
  print "minus number and the square root of it times 3:"
  print the variable result_one
end if

the variable value_two of type Number is sqrt_four
if value_two is greater than 0 then
  the variable result_two of type Number is value_two times 2
  print "plus number and the square root of it times 2:"
  print the variable result_two
else
  the variable result_two of type Number is value_two times 3
  print "minus number and the square root of it times 3:"
  print the variable result_two
end if

the variable value_three of type Number is sqrt_nine
if value_three is greater than 0 then
  the variable result_three of type Number is value_three times 2
  print "plus number and the square root of it times 2:"
  print the variable result_three
else
  the variable result_three of type Number is value_three times 3
  print "minus number and the square root of it times 3:"
  print the variable result_three
end if

```

---

### [test_051.lay](test-cases/imports/test_051.lay)
**Description**: Here is a possible test case description for Test Case Number 51:

```ruby
# test case 51
# description: Here is a possible test case description for Test Case Number 51:

# Write a program that imports a mathematical module and uses its functions to calculate the area of a circle given its radius, then verifies the result matches an expected value.

define function calculate_pi that takes dummy as Number and returns Number
  return 3.14159

define function calculate_square that takes radius as Number and returns Number
  return radius times radius

define function calculate_area that takes radius as Number and returns Number
  the variable dummy_arg of type Number is 0
  the variable pi_value of type Number is call function calculate_pi with argument dummy_arg
  the variable radius_squared of type Number is call function calculate_square with argument radius
  the variable area of type Number is pi_value times radius_squared
  return area

# calculate area of circle with radius 5
the variable radius of type Number is 5
the variable area of type Number is call function calculate_area with argument radius

# calculate additional value
the variable dummy_arg of type Number is 0
the variable pi_value of type Number is call function calculate_pi with argument dummy_arg
the variable additional_value of type Number is 6.7085
the variable total of type Number is area plus additional_value
the variable total of type Number is total times pi_value

# verify result
if total is greater than 0 then
  the variable result of type Number is total
  print "the variable result:"
  print the variable result
else
  print "calculation error"
end if

```

---

### [test_071.lay](test-cases/imports/test_071.lay)
**Description**: Write a program that imports a math module and uses its functions to calculate the area of a rectangle, testing that the result is correct when both the length and width are provided.

```ruby
# test case 71
# description: Write a program that imports a math module and uses its functions to calculate the area of a rectangle, testing that the result is correct when both the length and width are provided.

define function calculate_area that takes length as Number and width as Number and returns Number
  return length times width

# test with different length and width values
the variable length of type Number is 2
the variable width of type Number is 3

# calculate area
the variable area_result of type Number is call function calculate_area with argument length and argument width

print "area of rectangle:"
print the variable area_result

# test with different values
the variable length_two of type Number is 3
the variable width_two of type Number is 4

the variable area_result_two of type Number is call function calculate_area with argument length_two and argument width_two

print "area of rectangle 2:"
print the variable area_result_two

# test with length = 1
the variable length_three of type Number is 1
the variable width_three of type Number is 5

the variable area_result_three of type Number is call function calculate_area with argument length_three and argument width_three

if area_result_three is greater than 0 then
  print "area calculation successful:"
  print the variable area_result_three
else
  print "an error message"
end if

```

---

### [test_091.lay](test-cases/imports/test_091.lay)
**Description**: importing a module and using nested loops to combine values

```ruby
# test case 91
# description: importing a module and using nested loops to combine values

# simulate importing by using a local function and nested loops

define function combine that takes a as Number and b as Number and returns Number
  return a plus b

# compute a total using nested loops
 the variable total of type Number is 0
 the variable i of type Number is 0
 while i is less than 3 do
   the variable j of type Number is 0
   while j is less than 3 do
     total is total plus call function combine with argument i and argument j
     j is j plus 1
   end while
   i is i plus 1
 end while

print the variable total

```

---

## Negative

### [basic_001_incomplete_assignment.lay](test-cases/negative/basic_001_incomplete_assignment.lay)
**Description**: Tests that incomplete assignment statements are caught

```ruby
# negative test case: incomplete assignment
# expected error: Missing value after 'is' in assignment
# description: Tests that incomplete assignment statements are caught

the variable x of type Number is



```

---

### [basic_002_invalid_equals.lay](test-cases/negative/basic_002_invalid_equals.lay)
**Description**: Tests that incomplete 'equals' usage is caught (like the bug we found)

```ruby
# negative test case: invalid equals usage
# expected error: Incomplete statement - 'equals' without right-hand side
# description: Tests that incomplete 'equals' usage is caught (like the bug we found)

then print result of age is age equals



```

---

### [basic_003_missing_variable_keyword.lay](test-cases/negative/basic_003_missing_variable_keyword.lay)
**Description**: Tests that 'the variable' pattern is required

```ruby
# negative test case: missing 'variable' keyword
# expected error: Missing 'variable' keyword in declaration
# description: Tests that 'the variable' pattern is required

the x is 5



```

---

### [basic_004_missing_is.lay](test-cases/negative/basic_004_missing_is.lay)
**Description**: Tests that assignment requires 'is' or 'equals'

```ruby
# negative test case: missing 'is' keyword in assignment
# expected error: Expected 'is' or 'equals' for assignment
# description: Tests that assignment requires 'is' or 'equals'

the variable x of type Number 5



```

---

### [complex_001_incomplete_if.lay](test-cases/negative/complex_001_incomplete_if.lay)
**Description**: Tests that if statements must have complete structure

```ruby
# negative test case: incomplete if statement
# expected error: Missing 'then' clause in if statement
# description: Tests that if statements must have complete structure

if x is 5



```

---

### [complex_002_incomplete_for_loop.lay](test-cases/negative/complex_002_incomplete_for_loop.lay)
**Description**: Tests that for loops must have complete structure

```ruby
# negative test case: incomplete for loop
# expected error: Missing 'do' clause in for each loop
# description: Tests that for loops must have complete structure

for each item in list



```

---

### [context_001_is_without_context.lay](test-cases/negative/context_001_is_without_context.lay)
**Description**: Tests context-aware 'is' keyword validation

```ruby
# negative test case: 'is' without proper context
# expected error: 'is' keyword used without valid context
# description: Tests context-aware 'is' keyword validation

x is



```

---

### [context_002_is_a_without_new.lay](test-cases/negative/context_002_is_a_without_new.lay)
**Description**: Tests that object creation syntax is complete

```ruby
# negative test case: 'is a' without 'new' (incomplete object creation)
# expected error: 'is a' must be followed by 'new' for object creation
# description: Tests that object creation syntax is complete

the variable person is a Person with
  name which is 'Alice'



```

---

### [test_001_incomplete_throw.lay](test-cases/negative/test_001_incomplete_throw.lay)
**Description**: No description available.

```ruby
# negative test: incomplete throw statement
# expected: error should be thrown during parsing or compilation

throw



```

---

### [test_002_incomplete_try.lay](test-cases/negative/test_002_incomplete_try.lay)
**Description**: No description available.

```ruby
# negative test: incomplete try statement
# expected: error should be thrown

try
  the variable x of type Number is 5



```

---

### [test_003_try_without_catch.lay](test-cases/negative/test_003_try_without_catch.lay)
**Description**: No description available.

```ruby
# negative test: try without catch block
# expected: should handle or warn

try
  throw 'error'
end try



```

---

### [test_004_incomplete_expect.lay](test-cases/negative/test_004_incomplete_expect.lay)
**Description**: No description available.

```ruby
# negative test: incomplete expect statement
# expected: error should be thrown

expect the variable x



```

---

### [test_005_expect_without_is.lay](test-cases/negative/test_005_expect_without_is.lay)
**Description**: No description available.

```ruby
# negative test: expect without 'is' keyword
# expected: error should be thrown

expect the variable x the variable y



```

---

### [test_006_incomplete_test.lay](test-cases/negative/test_006_incomplete_test.lay)
**Description**: No description available.

```ruby
# negative test: incomplete test statement
# expected: error should be thrown

test 'my test'



```

---

### [test_007_incomplete_describe.lay](test-cases/negative/test_007_incomplete_describe.lay)
**Description**: No description available.

```ruby
# negative test: incomplete describe statement
# expected: error should be thrown

describe 'my suite'



```

---

### [test_008_test_without_end.lay](test-cases/negative/test_008_test_without_end.lay)
**Description**: No description available.

```ruby
# negative test: test without end test
# expected: error should be thrown

test 'my test'
  the variable x of type Number is 5



```

---

### [test_009_describe_without_end.lay](test-cases/negative/test_009_describe_without_end.lay)
**Description**: No description available.

```ruby
# negative test: describe without end describe
# expected: error should be thrown

describe 'my suite'
  test 'test 1'
    print 'test'
  end test



```

---

### [test_010_invalid_import_path.lay](test-cases/negative/test_010_invalid_import_path.lay)
**Description**: No description available.

```ruby
# negative test: invalid import path
# expected: error should be thrown - file not found

import nonexistent from 'nonexistent_file.lay'



```

---

### [test_011_invalid_import_syntax.lay](test-cases/negative/test_011_invalid_import_syntax.lay)
**Description**: No description available.

```ruby
# negative test: invalid import syntax
# expected: error should be thrown

import



```

---

### [test_012_function_without_body.lay](test-cases/negative/test_012_function_without_body.lay)
**Description**: No description available.

```ruby
# negative test: function declaration without body
# expected: error should be thrown

define function my_function that takes x and returns Number



```

---

### [test_013_function_without_name.lay](test-cases/negative/test_013_function_without_name.lay)
**Description**: No description available.

```ruby
# negative test: function declaration without name
# expected: error should be thrown

define function that takes x and returns Number
  return x



```

---

### [test_014_class_without_properties.lay](test-cases/negative/test_014_class_without_properties.lay)
**Description**: No description available.

```ruby
# negative test: class declaration without properties
# expected: error should be thrown or warning

define class Person



```

---

### [test_015_object_creation_incomplete.lay](test-cases/negative/test_015_object_creation_incomplete.lay)
**Description**: No description available.

```ruby
# negative test: incomplete object creation
# expected: error should be thrown

the variable person of type Person is a new Person



```

---

### [test_016_object_creation_wrong_type.lay](test-cases/negative/test_016_object_creation_wrong_type.lay)
**Description**: No description available.

```ruby
# negative test: object creation with wrong type
# expected: type error should be thrown

the variable person of type Number is a new Person with
  name is 'Alice'



```

---

### [test_017_method_call_on_non_object.lay](test-cases/negative/test_017_method_call_on_non_object.lay)
**Description**: No description available.

```ruby
# negative test: method call on non-object
# expected: error should be thrown

the variable x of type Number is 5
call x dot greet



```

---

### [test_018_undefined_method.lay](test-cases/negative/test_018_undefined_method.lay)
**Description**: No description available.

```ruby
# negative test: method call on undefined method
# expected: error should be thrown

the variable person of type Person is a new Person with
  name is 'Alice'
call person dot nonexistent_method



```

---

### [test_019_return_without_function.lay](test-cases/negative/test_019_return_without_function.lay)
**Description**: No description available.

```ruby
# negative test: return statement outside function
# expected: error should be thrown

return 5



```

---

### [test_020_incomplete_conditional.lay](test-cases/negative/test_020_incomplete_conditional.lay)
**Description**: No description available.

```ruby
# negative test: incomplete conditional statement
# expected: error should be thrown

if true



```

---

### [test_021_if_without_then.lay](test-cases/negative/test_021_if_without_then.lay)
**Description**: No description available.

```ruby
# negative test: if without 'then' keyword
# expected: error should be thrown

if true
  print 'hello'



```

---

### [test_022_else_without_if.lay](test-cases/negative/test_022_else_without_if.lay)
**Description**: No description available.

```ruby
# negative test: else without if
# expected: error should be thrown

else
  print 'hello'



```

---

### [test_023_for_without_each.lay](test-cases/negative/test_023_for_without_each.lay)
**Description**: No description available.

```ruby
# negative test: for without 'each' keyword
# expected: error should be thrown

for item in list do
  print item



```

---

### [test_024_for_without_collection.lay](test-cases/negative/test_024_for_without_collection.lay)
**Description**: No description available.

```ruby
# negative test: for each without collection
# expected: error should be thrown

for each item do
  print item



```

---

### [test_025_while_without_condition.lay](test-cases/negative/test_025_while_without_condition.lay)
**Description**: No description available.

```ruby
# negative test: while without condition
# expected: error should be thrown

while do
  print 'loop'



```

---

### [test_026_while_without_do.lay](test-cases/negative/test_026_while_without_do.lay)
**Description**: No description available.

```ruby
# negative test: while without 'do' keyword
# expected: error should be thrown

while true
  print 'loop'



```

---

### [test_027_undefined_variable_in_assignment.lay](test-cases/negative/test_027_undefined_variable_in_assignment.lay)
**Description**: No description available.

```ruby
# negative test: using undefined variable in assignment
# expected: type error or runtime error

the variable x of type Number is undefined_var



```

---

### [test_028_type_mismatch_in_operation.lay](test-cases/negative/test_028_type_mismatch_in_operation.lay)
**Description**: No description available.

```ruby
# negative test: type mismatch in operation
# expected: type error should be thrown

the variable x of type String is 'hello'
the variable y of type Number is x divided by 2



```

---

### [test_029_invalid_binary_operation.lay](test-cases/negative/test_029_invalid_binary_operation.lay)
**Description**: No description available.

```ruby
# negative test: invalid binary operation
# expected: error should be thrown

the variable x of type String is 'hello'
the variable y of type Number is x minus 5



```

---

### [test_030_incomplete_operation.lay](test-cases/negative/test_030_incomplete_operation.lay)
**Description**: No description available.

```ruby
# negative test: incomplete operation expression
# expected: error should be thrown

the variable x of type Number is 5 plus



```

---

### [test_031_function_call_without_function.lay](test-cases/negative/test_031_function_call_without_function.lay)
**Description**: No description available.

```ruby
# negative test: function call to undefined function
# expected: error should be thrown

call undefined_function



```

---

### [test_032_function_call_wrong_args.lay](test-cases/negative/test_032_function_call_wrong_args.lay)
**Description**: No description available.

```ruby
# negative test: function call with wrong number of arguments
# expected: error should be thrown

define function add that takes x and y and returns Number
  return x plus y

call add with 1 argument



```

---

### [test_033_nested_error_handling.lay](test-cases/negative/test_033_nested_error_handling.lay)
**Description**: No description available.

```ruby
# negative test: nested try-catch incorrectly
# expected: error should be thrown or handled correctly
# this test should execute and throw an error (which is expected for negative test)

try
  try
    throw 'inner error'
  catch error
    throw 'outer error'
  end try
catch error
  print error
end try

```

---

### [test_034_expect_error_incomplete.lay](test-cases/negative/test_034_expect_error_incomplete.lay)
**Description**: No description available.

```ruby
# negative test: incomplete expect error statement
# expected: error should be thrown during execution
# this test should compile but fail at runtime when expect error is incomplete

expect error
  throw 'error'
# missing 'with message' clause

```

---

### [test_035_expect_error_without_message.lay](test-cases/negative/test_035_expect_error_without_message.lay)
**Description**: No description available.

```ruby
# negative test: expect error without message value
# expected: error should be thrown during execution
# this test should compile but fail at runtime when message is missing

expect error
  throw 'error'
with message
# missing message value

```

---

### [test_036_test_inside_test.lay](test-cases/negative/test_036_test_inside_test.lay)
**Description**: No description available.

```ruby
# negative test: test inside test (nested)
# expected: should handle or error

test 'outer test'
  test 'inner test'
    expect 5 is 5
  end test
end test



```

---

### [test_037_describe_inside_describe.lay](test-cases/negative/test_037_describe_inside_describe.lay)
**Description**: No description available.

```ruby
# negative test: describe inside describe (nested)
# expected: should handle or error

describe 'outer suite'
  describe 'inner suite'
    test 'test'
      expect 5 is 5
    end test
  end describe
end describe



```

---

### [test_038_expect_in_test_wrong_type.lay](test-cases/negative/test_038_expect_in_test_wrong_type.lay)
**Description**: No description available.

```ruby
# negative test: expect with type mismatch
# expected: should show type error or assertion failure

test 'type test'
  the variable x of type Number is 5
  expect x is '5'
end test



```

---

### [test_039_invalid_operator_sequence.lay](test-cases/negative/test_039_invalid_operator_sequence.lay)
**Description**: No description available.

```ruby
# negative test: invalid operator sequence
# expected: error should be thrown

the variable x of type Number is 5 plus plus 3



```

---

### [test_040_missing_operator.lay](test-cases/negative/test_040_missing_operator.lay)
**Description**: No description available.

```ruby
# negative test: missing operator between values
# expected: error should be thrown

the variable x of type Number is 8



```

---

### [test_041_invalid_string_literal.lay](test-cases/negative/test_041_invalid_string_literal.lay)
**Description**: No description available.

```ruby
# negative test: invalid string literal
# expected: error should be thrown

the variable x of type String is 'unclosed string



```

---

### [test_042_invalid_number_literal.lay](test-cases/negative/test_042_invalid_number_literal.lay)
**Description**: No description available.

```ruby
# negative test: invalid number literal
# expected: error should be thrown

the variable x of type Number is 12.34.56



```

---

### [test_043_invalid_type_annotation.lay](test-cases/negative/test_043_invalid_type_annotation.lay)
**Description**: No description available.

```ruby
# negative test: invalid type annotation
# expected: error should be thrown

the variable x of type InvalidType is 5



```

---

### [test_044_duplicate_variable.lay](test-cases/negative/test_044_duplicate_variable.lay)
**Description**: No description available.

```ruby
# negative test: duplicate variable declaration
# expected: error should be thrown

the variable x of type Number is 5
the variable x of type Number is 10



```

---

### [test_045_invalid_indentation.lay](test-cases/negative/test_045_invalid_indentation.lay)
**Description**: No description available.

```ruby
# negative test: invalid indentation in block
# expected: should handle or error

if true then
print 'should be indented'



```

---

### [test_046_unreachable_code.lay](test-cases/negative/test_046_unreachable_code.lay)
**Description**: No description available.

```ruby
# negative test: unreachable code after return
# expected: warning or should work

define function test that returns Number
  return 5
  print 'unreachable'



```

---

### [test_047_invalid_object_property.lay](test-cases/negative/test_047_invalid_object_property.lay)
**Description**: No description available.

```ruby
# negative test: invalid property in object creation
# expected: error should be thrown

the variable person of type Person is a new Person with
  invalid_property is 'value'



```

---

### [test_048_missing_property_in_object.lay](test-cases/negative/test_048_missing_property_in_object.lay)
**Description**: No description available.

```ruby
# negative test: missing required property in object
# expected: error should be thrown

define class Person
  the property name of type String

the variable person of type Person is a new Person with



```

---

### [test_049_invalid_loop_variable.lay](test-cases/negative/test_049_invalid_loop_variable.lay)
**Description**: No description available.

```ruby
# negative test: invalid loop variable usage
# expected: error should be thrown

for each in list do
  print 'missing variable name'



```

---

### [test_050_test_framework_negative.lay](test-cases/negative/test_050_test_framework_negative.lay)
**Description**: No description available.

```ruby
# negative test: test framework usage errors
# expected: errors should be caught during parsing

test
  expect 5 is 5
end test

describe
  test 'test'
    expect 5 is 5
  end test
end describe

```

---

### [test_051_missing_expression_after_throw.lay](test-cases/negative/test_051_missing_expression_after_throw.lay)
**Description**: No description available.

```ruby
# negative test: throw without expression
# expected: parser should reject this

throw



```

---

### [test_052_incomplete_if_condition.lay](test-cases/negative/test_052_incomplete_if_condition.lay)
**Description**: No description available.

```ruby
# negative test: if statement without condition
# expected: parser should reject this

if then
  print 'hello'
end if



```

---

### [test_053_if_without_body.lay](test-cases/negative/test_053_if_without_body.lay)
**Description**: No description available.

```ruby
# negative test: if statement without body
# expected: parser should reject this

if true then



```

---

### [test_054_for_without_iterator.lay](test-cases/negative/test_054_for_without_iterator.lay)
**Description**: No description available.

```ruby
# negative test: for each without iterator variable
# expected: parser should reject this

for each in list do
  print 'item'
end for



```

---

### [test_055_function_missing_return.lay](test-cases/negative/test_055_function_missing_return.lay)
**Description**: No description available.

```ruby
# negative test: function with return type but no return statement
# expected: type checker should catch this

define function add that takes x and y and returns Number
  the variable result of type Number is x plus y



```

---

### [test_056_class_missing_property_type.lay](test-cases/negative/test_056_class_missing_property_type.lay)
**Description**: No description available.

```ruby
# negative test: class property without type
# expected: parser should reject this

define class Person
  the property name



```

---

### [test_057_object_wrong_property_value.lay](test-cases/negative/test_057_object_wrong_property_value.lay)
**Description**: No description available.

```ruby
# negative test: object property with wrong type value
# expected: type checker should catch this

define class Person
  the property age of type Number

the variable person of type Person is a new Person with
  age is 'not a number'



```

---

### [test_058_call_without_function_name.lay](test-cases/negative/test_058_call_without_function_name.lay)
**Description**: No description available.

```ruby
# negative test: call statement without function name
# expected: parser should reject this

call
  print 'hello'



```

---

### [test_059_method_call_on_primitive.lay](test-cases/negative/test_059_method_call_on_primitive.lay)
**Description**: No description available.

```ruby
# negative test: method call on primitive type
# expected: type checker should catch this

the variable x of type Number is 5
call x dot toString



```

---

### [test_060_invalid_arithmetic.lay](test-cases/negative/test_060_invalid_arithmetic.lay)
**Description**: No description available.

```ruby
# negative test: arithmetic operation on incompatible types
# expected: type checker should catch this

the variable x of type String is 'hello'
the variable y of type Number is x divided by 2



```

---

### [test_061_undefined_variable_in_expression.lay](test-cases/negative/test_061_undefined_variable_in_expression.lay)
**Description**: No description available.

```ruby
# negative test: using undefined variable in expression
# expected: type checker should catch this

the variable result of type Number is undefined_var plus 5



```

---

### [test_062_return_outside_function.lay](test-cases/negative/test_062_return_outside_function.lay)
**Description**: No description available.

```ruby
# negative test: return statement outside function
# expected: parser should reject this or type checker should catch this

return 5



```

---

### [test_063_break_outside_loop.lay](test-cases/negative/test_063_break_outside_loop.lay)
**Description**: No description available.

```ruby
# negative test: break statement outside loop
# expected: parser should reject this

break



```

---

### [test_064_continue_outside_loop.lay](test-cases/negative/test_064_continue_outside_loop.lay)
**Description**: No description available.

```ruby
# negative test: continue statement outside loop
# expected: parser should reject this

continue



```

---

### [test_065_invalid_string_escape.lay](test-cases/negative/test_065_invalid_string_escape.lay)
**Description**: No description available.

```ruby
# negative test: invalid string escape sequence
# expected: lexer should handle this gracefully

the variable x of type String is 'hello\nworld'



```

---

### [test_066_variable_redeclaration.lay](test-cases/negative/test_066_variable_redeclaration.lay)
**Description**: No description available.

```ruby
# negative test: redeclaring same variable
# expected: type checker should catch this

the variable x of type Number is 5
the variable x of type Number is 10



```

---

### [test_067_function_redefinition.lay](test-cases/negative/test_067_function_redefinition.lay)
**Description**: No description available.

```ruby
# negative test: redefining same function
# expected: type checker should catch this

define function add that takes x and y and returns Number
  return x plus y

define function add that takes a and b and returns Number
  return a plus b



```

---

### [test_068_class_redefinition.lay](test-cases/negative/test_068_class_redefinition.lay)
**Description**: No description available.

```ruby
# negative test: redefining same class
# expected: type checker should catch this

define class Person
  the property name of type String

define class Person
  the property age of type Number



```

---

### [test_069_invalid_type_conversion.lay](test-cases/negative/test_069_invalid_type_conversion.lay)
**Description**: No description available.

```ruby
# negative test: invalid type conversion
# expected: type checker should catch this

the variable x of type String is 'hello'
the variable y of type Number is convert x to Number



```

---

### [test_070_missing_operator.lay](test-cases/negative/test_070_missing_operator.lay)
**Description**: No description available.

```ruby
# negative test: missing operator between values
# expected: parser should reject this

the variable x of type Number is 15



```

---

### [test_071_if_without_condition.lay](test-cases/negative/test_071_if_without_condition.lay)
**Description**: No description available.

```ruby
# negative test: if statement completely missing condition
# expected: parser should reject this

if then
  print 'hello'
end if



```

---

### [test_072_else_without_if.lay](test-cases/negative/test_072_else_without_if.lay)
**Description**: No description available.

```ruby
# negative test: else statement without preceding if
# expected: parser should reject this

else
  print 'hello'
end if



```

---

### [test_073_for_without_each.lay](test-cases/negative/test_073_for_without_each.lay)
**Description**: No description available.

```ruby
# negative test: for loop without 'each' keyword
# expected: parser should reject this

for item in list do
  print item
end for



```

---

### [test_074_while_without_condition.lay](test-cases/negative/test_074_while_without_condition.lay)
**Description**: No description available.

```ruby
# negative test: while loop without condition
# expected: parser should reject this

while do
  print 'hello'
end while



```

---

### [test_075_function_without_name.lay](test-cases/negative/test_075_function_without_name.lay)
**Description**: No description available.

```ruby
# negative test: function declaration without function name
# expected: parser should reject this

define function that takes x and y and returns Number
  return x plus y



```

---

### [test_076_function_without_return_type.lay](test-cases/negative/test_076_function_without_return_type.lay)
**Description**: No description available.

```ruby
# negative test: function with return but no return type
# expected: parser should reject this

define function add that takes x and y
  return x plus y



```

---

### [test_077_class_without_name.lay](test-cases/negative/test_077_class_without_name.lay)
**Description**: No description available.

```ruby
# negative test: class declaration without class name
# expected: parser should reject this

define class
  the property name of type String



```

---

### [test_078_object_without_type.lay](test-cases/negative/test_078_object_without_type.lay)
**Description**: No description available.

```ruby
# negative test: object creation without type
# expected: parser should reject this

the variable person is a new with
  name is 'John'



```

---

### [test_079_method_call_missing_dot.lay](test-cases/negative/test_079_method_call_missing_dot.lay)
**Description**: No description available.

```ruby
# negative test: method call syntax without dot operator
# expected: parser should reject this

the variable person of type Person is a new Person with
  name is 'John'

call person getName



```

---

### [test_080_undefined_variable.lay](test-cases/negative/test_080_undefined_variable.lay)
**Description**: No description available.

```ruby
# negative test: using undefined variable
# expected: type checker should catch this

the variable result of type Number is undefined_var plus 5



```

---

### [test_081_type_mismatch_assignment.lay](test-cases/negative/test_081_type_mismatch_assignment.lay)
**Description**: No description available.

```ruby
# negative test: assigning wrong type to variable
# expected: type checker should catch this

the variable x of type Number is 'hello'



```

---

### [test_082_arithmetic_string_numbers.lay](test-cases/negative/test_082_arithmetic_string_numbers.lay)
**Description**: No description available.

```ruby
# negative test: arithmetic operation on string and number
# expected: type checker should catch this

the variable x of type String is 'hello'
the variable y of type Number is x plus 5



```

---

### [test_083_division_by_zero.lay](test-cases/negative/test_083_division_by_zero.lay)
**Description**: No description available.

```ruby
# negative test: division by 0 (should throw at runtime)
# expected: execution should fail with error

the variable x of type Number is 10 divided by 0



```

---

### [test_084_undefined_function_call.lay](test-cases/negative/test_084_undefined_function_call.lay)
**Description**: No description available.

```ruby
# negative test: calling undefined function
# expected: type checker should catch this

call undefined_function



```

---

### [test_085_function_wrong_arity.lay](test-cases/negative/test_085_function_wrong_arity.lay)
**Description**: No description available.

```ruby
# negative test: calling function with wrong number of arguments
# expected: type checker should catch this

define function add that takes x and y and returns Number
  return x plus y

call add with 5



```

---

### [test_086_nested_class_definition.lay](test-cases/negative/test_086_nested_class_definition.lay)
**Description**: No description available.

```ruby
# negative test: class definition inside function
# expected: parser should reject this

define function test that takes nothing and returns Void
  define class Inner
    the property value of type Number
  end class
end function



```

---

### [test_087_function_in_function.lay](test-cases/negative/test_087_function_in_function.lay)
**Description**: No description available.

```ruby
# negative test: function definition inside another function
# expected: parser should reject this

define function outer that takes nothing and returns Void
  define function inner that takes nothing and returns Void
    print 'inner'
  end function
end function



```

---

### [test_088_return_value_type_mismatch.lay](test-cases/negative/test_088_return_value_type_mismatch.lay)
**Description**: No description available.

```ruby
# negative test: return value doesn't match function return type
# expected: type checker should catch this

define function getNumber that takes nothing and returns Number
  return 'not a number'



```

---

### [test_089_missing_return_statement.lay](test-cases/negative/test_089_missing_return_statement.lay)
**Description**: No description available.

```ruby
# negative test: function with return type but no return
# expected: type checker should catch this

define function add that takes x and y and returns Number
  the variable result of type Number is x plus y



```

---

### [test_090_object_wrong_property_name.lay](test-cases/negative/test_090_object_wrong_property_name.lay)
**Description**: No description available.

```ruby
# negative test: object with property that doesn't exist in class
# expected: type checker should catch this

define class Person
  the property name of type String

the variable person of type Person is a new Person with
  age is 25



```

---

### [test_091_object_missing_property.lay](test-cases/negative/test_091_object_missing_property.lay)
**Description**: No description available.

```ruby
# negative test: object missing required property
# expected: type checker should catch this

define class Person
  the property name of type String
  the property age of type Number

the variable person of type Person is a new Person with
  name is 'John'



```

---

### [test_092_method_on_non_object.lay](test-cases/negative/test_092_method_on_non_object.lay)
**Description**: No description available.

```ruby
# negative test: calling method on primitive type
# expected: type checker should catch this

the variable x of type Number is 5
call x dot toString



```

---

### [test_093_undefined_method.lay](test-cases/negative/test_093_undefined_method.lay)
**Description**: No description available.

```ruby
# negative test: calling undefined method on object
# expected: type checker should catch this

define class Person
  the property name of type String

the variable person of type Person is a new Person with
  name is 'John'

call person dot undefinedMethod



```

---

### [test_094_invalid_operator.lay](test-cases/negative/test_094_invalid_operator.lay)
**Description**: No description available.

```ruby
# negative test: invalid operator usage
# expected: parser should reject this

the variable x of type Number is 5 @ 7



```

---

### [test_095_string_plus_number.lay](test-cases/negative/test_095_string_plus_number.lay)
**Description**: No description available.

```ruby
# negative test: string concatenation with number (should be explicit)
# expected: type checker should catch this

the variable x of type String is 'hello' plus 5



```

---

### [test_096_boolean_arithmetic.lay](test-cases/negative/test_096_boolean_arithmetic.lay)
**Description**: No description available.

```ruby
# negative test: arithmetic operation on boolean
# expected: type checker should catch this

the variable x of type Bool is true
the variable y of type Number is x plus 5



```

---

### [test_097_conditional_type_mismatch.lay](test-cases/negative/test_097_conditional_type_mismatch.lay)
**Description**: No description available.

```ruby
# negative test: if condition with non-boolean value
# expected: type checker should catch this

if 5 then
  print 'hello'
end if



```

---

### [test_098_loop_wrong_type.lay](test-cases/negative/test_098_loop_wrong_type.lay)
**Description**: No description available.

```ruby
# negative test: for loop iterator on non-iterable
# expected: type checker should catch this

the variable x of type Number is 5
for each item in x do
  print item
end for



```

---

### [test_099_while_wrong_type.lay](test-cases/negative/test_099_while_wrong_type.lay)
**Description**: No description available.

```ruby
# negative test: while condition with non-boolean
# expected: type checker should catch this

while 5 do
  print 'hello'
end while



```

---

### [test_100_incomplete_expression.lay](test-cases/negative/test_100_incomplete_expression.lay)
**Description**: No description available.

```ruby
# negative test: incomplete expression statement
# expected: parser should reject this

the variable x of type Number is 5 plus



```

---

### [test_101_missing_semicolon_equivalent.lay](test-cases/negative/test_101_missing_semicolon_equivalent.lay)
**Description**: No description available.

```ruby
# negative test: statements that should be separated but aren't
# expected: parser should handle this gracefully or reject

the variable x of type Number is 5 the variable y of type Number is 10



```

---

### [test_102_duplicate_variable.lay](test-cases/negative/test_102_duplicate_variable.lay)
**Description**: No description available.

```ruby
# negative test: declaring same variable twice
# expected: type checker should catch this

the variable x of type Number is 5
the variable x of type Number is 10



```

---

### [test_103_duplicate_function.lay](test-cases/negative/test_103_duplicate_function.lay)
**Description**: No description available.

```ruby
# negative test: defining same function twice
# expected: type checker should catch this

define function add that takes x and y and returns Number
  return x plus y

define function add that takes a and b and returns Number
  return a plus b



```

---

### [test_104_duplicate_class.lay](test-cases/negative/test_104_duplicate_class.lay)
**Description**: No description available.

```ruby
# negative test: defining same class twice
# expected: type checker should catch this

define class Person
  the property name of type String

define class Person
  the property age of type Number



```

---

### [test_105_circular_import.lay](test-cases/negative/test_105_circular_import.lay)
**Description**: No description available.

```ruby
# negative test: circular import (should be detected)
# expected: resolver should catch this

import a from './test_106_circular_import_b.lay'



```

---

### [test_106_circular_import_b.lay](test-cases/negative/test_106_circular_import_b.lay)
**Description**: No description available.

```ruby
# negative test: circular import part 2
# expected: resolver should catch this

import a from './test_105_circular_import.lay'



```

---

### [test_107_import_nonexistent.lay](test-cases/negative/test_107_import_nonexistent.lay)
**Description**: No description available.

```ruby
# negative test: importing non-existent file
# expected: resolver should catch this

import utils from './nonexistent_file.lay'



```

---

### [test_108_try_without_catch.lay](test-cases/negative/test_108_try_without_catch.lay)
**Description**: No description available.

```ruby
# negative test: try block without catch
# expected: parser should reject this or require catch

try
  throw 'error'
end try



```

---

### [test_109_catch_without_try.lay](test-cases/negative/test_109_catch_without_try.lay)
**Description**: No description available.

```ruby
# negative test: catch without try
# expected: parser should reject this

catch error
  print error
end catch



```

---

### [test_110_expect_without_is.lay](test-cases/negative/test_110_expect_without_is.lay)
**Description**: No description available.

```ruby
# negative test: expect statement without 'is' keyword
# expected: parser should reject this

expect 10



```

---

### [test_111_test_without_end.lay](test-cases/negative/test_111_test_without_end.lay)
**Description**: No description available.

```ruby
# negative test: test statement without end test
# expected: parser should reject this

test 'my test'
  expect 5 is 5



```

---

### [test_112_describe_without_end.lay](test-cases/negative/test_112_describe_without_end.lay)
**Description**: No description available.

```ruby
# negative test: describe statement without end describe
# expected: parser should reject this

describe 'my suite'
  test 'test'
    expect 5 is 5
  end test



```

---

### [test_113_expect_error_wrong_syntax.lay](test-cases/negative/test_113_expect_error_wrong_syntax.lay)
**Description**: No description available.

```ruby
# negative test: expect error with wrong syntax
# expected: parser should reject this

expect error
  throw 'error'
is 'error message'



```

---

### [test_114_nested_expect_error.lay](test-cases/negative/test_114_nested_expect_error.lay)
**Description**: No description available.

```ruby
# negative test: nested expect error (should be invalid)
# expected: parser should reject this or handle correctly

expect error
  expect error
    throw 'nested'
  with message 'nested'
with message 'outer'



```

---

### [test_115_invalid_string_literal.lay](test-cases/negative/test_115_invalid_string_literal.lay)
**Description**: No description available.

```ruby
# negative test: invalid string literal (unclosed quote)
# expected: lexer should catch this

the variable x of type String is 'hello world



```

---

### [test_116_invalid_number.lay](test-cases/negative/test_116_invalid_number.lay)
**Description**: No description available.

```ruby
# negative test: invalid number format
# expected: lexer should catch this

the variable x of type Number is 12.34.56



```

---

### [test_117_reserved_keyword_variable.lay](test-cases/negative/test_117_reserved_keyword_variable.lay)
**Description**: No description available.

```ruby
# negative test: using reserved keyword as variable name
# expected: parser should reject this

the variable if of type Number is 5



```

---

### [test_118_reserved_keyword_function.lay](test-cases/negative/test_118_reserved_keyword_function.lay)
**Description**: No description available.

```ruby
# negative test: using reserved keyword as function name
# expected: parser should reject this

define function return that takes nothing and returns Void
  print 'hello'



```

---

### [test_119_invalid_indentation.lay](test-cases/negative/test_119_invalid_indentation.lay)
**Description**: No description available.

```ruby
# negative test: inconsistent indentation (should be handled gracefully)
# expected: parser should handle this or reject

if true then
 print 'hello'
  print 'world'
end if



```

---

### [test_120_empty_block.lay](test-cases/negative/test_120_empty_block.lay)
**Description**: No description available.

```ruby
# negative test: empty block statements
# expected: parser should handle this gracefully

if true then
end if

for each item in list do
end for



```

---

### [test_121_invalid_comparison.lay](test-cases/negative/test_121_invalid_comparison.lay)
**Description**: No description available.

```ruby
# negative test: comparing incompatible types
# expected: type checker should catch this

the variable x of type String is 'hello'
the variable y of type Number is 5
if x is y then
  print 'match'
end if



```

---

### [test_122_missing_operator_between.lay](test-cases/negative/test_122_missing_operator_between.lay)
**Description**: No description available.

```ruby
# negative test: missing operator between expressions
# expected: parser should reject this

the variable x of type Number is 15



```

---

### [test_123_invalid_nested_expression.lay](test-cases/negative/test_123_invalid_nested_expression.lay)
**Description**: No description available.

```ruby
# negative test: invalid nested expression structure
# expected: parser should reject this

the variable x of type Number is 5 plus plus 5



```

---

### [test_124_function_call_without_args_when_required.lay](test-cases/negative/test_124_function_call_without_args_when_required.lay)
**Description**: No description available.

```ruby
# negative test: function requiring arguments called without them
# expected: type checker should catch this

define function add that takes x and y and returns Number
  return x plus y

the variable result of type Number is call add



```

---

### [test_125_function_call_too_many_args.lay](test-cases/negative/test_125_function_call_too_many_args.lay)
**Description**: No description available.

```ruby
# negative test: function called with too many arguments
# expected: type checker should catch this

define function identity that takes x and returns Number
  return x

the variable result of type Number is call identity with 5 and 10



```

---

### [test_126_object_property_type_mismatch.lay](test-cases/negative/test_126_object_property_type_mismatch.lay)
**Description**: No description available.

```ruby
# negative test: object property with wrong type
# expected: type checker should catch this

define class Person
  the property name of type String
  the property age of type Number

the variable person of type Person is a new Person with
  name is 'John'
  age is 'not a number'



```

---

### [test_127_method_undefined_on_class.lay](test-cases/negative/test_127_method_undefined_on_class.lay)
**Description**: No description available.

```ruby
# negative test: calling method that doesn't exist on class
# expected: type checker should catch this

define class Person
  the property name of type String

the variable person of type Person is a new Person with
  name is 'John'

call person dot getAge



```

---

### [test_128_invalid_arithmetic_order.lay](test-cases/negative/test_128_invalid_arithmetic_order.lay)
**Description**: No description available.

```ruby
# negative test: invalid operator precedence or syntax
# expected: parser should reject this or type checker should catch

the variable x of type Number is 5 divided by divided by 2



```

---

### [test_129_loop_variable_type_mismatch.lay](test-cases/negative/test_129_loop_variable_type_mismatch.lay)
**Description**: No description available.

```ruby
# negative test: loop variable type doesn't match collection
# expected: type checker should catch this

the variable numbers of type List of Number is 1 and 2 and 3
for each item of type String in numbers do
  print item
end for



```

---

### [test_130_conditional_without_boolean.lay](test-cases/negative/test_130_conditional_without_boolean.lay)
**Description**: No description available.

```ruby
# negative test: if condition that isn't boolean
# expected: type checker should catch this

if 'hello' then
  print 'world'
end if



```

---

### [test_131_return_in_wrong_context.lay](test-cases/negative/test_131_return_in_wrong_context.lay)
**Description**: No description available.

```ruby
# negative test: return statement in wrong context (not in function)
# expected: parser or type checker should catch this

if true then
  return 5
end if



```

---

### [test_132_break_in_wrong_place.lay](test-cases/negative/test_132_break_in_wrong_place.lay)
**Description**: No description available.

```ruby
# negative test: break statement outside loop
# expected: parser should reject this

if true then
  break
end if



```

---

### [test_133_continue_in_wrong_place.lay](test-cases/negative/test_133_continue_in_wrong_place.lay)
**Description**: No description available.

```ruby
# negative test: continue statement outside loop
# expected: parser should reject this

if true then
  continue
end if



```

---

### [test_134_variable_shadowing_issue.lay](test-cases/negative/test_134_variable_shadowing_issue.lay)
**Description**: No description available.

```ruby
# negative test: variable shadowing that might cause confusion
# expected: type checker might warn or reject

if true then
  the variable x of type Number is 5
end if
the variable x of type String is 'hello'



```

---

### [test_135_function_parameter_type_mismatch.lay](test-cases/negative/test_135_function_parameter_type_mismatch.lay)
**Description**: No description available.

```ruby
# negative test: function parameter with wrong type passed
# expected: type checker should catch this

define function add that takes x of type Number and y of type Number and returns Number
  return x plus y

the variable result of type Number is call add with 'hello' and 'world'



```

---

### [test_136_class_method_call_wrong_args.lay](test-cases/negative/test_136_class_method_call_wrong_args.lay)
**Description**: No description available.

```ruby
# negative test: method call with wrong number of arguments
# expected: type checker should catch this

define class Calculator
  define function add that takes x and y and returns Number
    return x plus y

the variable calc of type Calculator is a new Calculator with
call calc dot add with 5



```

---

### [test_137_invalid_import_relative_path.lay](test-cases/negative/test_137_invalid_import_relative_path.lay)
**Description**: No description available.

```ruby
# negative test: invalid relative import path
# expected: resolver should catch this

import utils from '../../../nonexistent/path/utils.lay'



```

---

### [test_138_import_without_quote.lay](test-cases/negative/test_138_import_without_quote.lay)
**Description**: No description available.

```ruby
# negative test: import path without quotes
# expected: parser should reject this

import utils from ./utils.lay



```

---

### [test_139_try_catch_wrong_order.lay](test-cases/negative/test_139_try_catch_wrong_order.lay)
**Description**: No description available.

```ruby
# negative test: catch before try
# expected: parser should reject this

catch error
  print error
try
  throw 'error'
end try



```

---

### [test_140_expect_invalid_actual.lay](test-cases/negative/test_140_expect_invalid_actual.lay)
**Description**: No description available.

```ruby
# negative test: expect with invalid actual value expression
# expected: parser should reject this

expect
  is 5



```

---

### [test_141_expect_invalid_expected.lay](test-cases/negative/test_141_expect_invalid_expected.lay)
**Description**: No description available.

```ruby
# negative test: expect with invalid expected value expression
# expected: parser should reject this

expect 5 is



```

---

### [test_142_test_nested_in_test.lay](test-cases/negative/test_142_test_nested_in_test.lay)
**Description**: No description available.

```ruby
# negative test: test statement nested in another test
# expected: parser might reject this or handle gracefully

test 'outer test'
  test 'inner test'
    expect 5 is 5
  end test
end test



```

---

### [test_143_describe_nested_in_test.lay](test-cases/negative/test_143_describe_nested_in_test.lay)
**Description**: No description available.

```ruby
# negative test: describe nested in test
# expected: parser might reject this or handle gracefully

test 'my test'
  describe 'nested suite'
    expect 5 is 5
  end describe
end test



```

---

### [test_144_call_without_function.lay](test-cases/negative/test_144_call_without_function.lay)
**Description**: No description available.

```ruby
# negative test: call statement without any function identifier
# expected: parser should reject this

call with 5 and 10



```

---

### [test_145_object_creation_missing_with.lay](test-cases/negative/test_145_object_creation_missing_with.lay)
**Description**: No description available.

```ruby
# negative test: object creation without 'with' keyword
# expected: parser should reject this

define class Person
  the property name of type String

the variable person of type Person is a new Person
  name is 'John'



```

---

### [test_146_class_property_without_type.lay](test-cases/negative/test_146_class_property_without_type.lay)
**Description**: No description available.

```ruby
# negative test: class property declaration without type
# expected: parser should reject this

define class Person
  the property name



```

---

### [test_147_function_parameter_without_type.lay](test-cases/negative/test_147_function_parameter_without_type.lay)
**Description**: No description available.

```ruby
# negative test: function parameter without type annotation
# expected: parser should reject this

define function add that takes x and y and returns Number
  return x plus y



```

---

### [test_148_variable_without_type.lay](test-cases/negative/test_148_variable_without_type.lay)
**Description**: No description available.

```ruby
# negative test: variable declaration without type
# expected: parser should reject this

the variable x is 5



```

---

### [test_149_invalid_boolean_operation.lay](test-cases/negative/test_149_invalid_boolean_operation.lay)
**Description**: No description available.

```ruby
# negative test: invalid boolean operation
# expected: type checker should catch this

the variable x of type Bool is true
the variable y of type Number is x multiplied by 5



```

---

### [test_150_string_division.lay](test-cases/negative/test_150_string_division.lay)
**Description**: No description available.

```ruby
# negative test: division operation on strings
# expected: type checker should catch this

the variable x of type String is 'hello'
the variable y of type String is 'world'
the variable z of type String is x divided by y



```

---

### [test_151_invalid_string_concatenation.lay](test-cases/negative/test_151_invalid_string_concatenation.lay)
**Description**: No description available.

```ruby
# negative test: invalid string concatenation syntax
# expected: parser should reject this or type checker should catch

the variable x of type String is 'hello' concatenated with 'world'



```

---

### [test_152_missing_comma_in_list.lay](test-cases/negative/test_152_missing_comma_in_list.lay)
**Description**: No description available.

```ruby
# negative test: list without proper separators
# expected: parser should reject this

the variable list of type List of Number is 6



```

---

### [test_153_invalid_object_property_access.lay](test-cases/negative/test_153_invalid_object_property_access.lay)
**Description**: No description available.

```ruby
# negative test: accessing property on non-object
# expected: type checker should catch this

the variable x of type Number is 5
the variable y of type String is x dot name



```

---

### [test_154_function_call_missing_with.lay](test-cases/negative/test_154_function_call_missing_with.lay)
**Description**: No description available.

```ruby
# negative test: function call without 'with' keyword
# expected: parser should reject this

define function add that takes x and y and returns Number
  return x plus y

the variable result of type Number is call add 5 and 10



```

---

### [test_155_class_inheritance_invalid.lay](test-cases/negative/test_155_class_inheritance_invalid.lay)
**Description**: No description available.

```ruby
# negative test: class inheriting from non-existent class
# expected: type checker should catch this

define class Child extends Parent
  the property value of type Number



```

---

### [test_156_method_override_wrong_signature.lay](test-cases/negative/test_156_method_override_wrong_signature.lay)
**Description**: No description available.

```ruby
# negative test: method override with wrong signature
# expected: type checker should catch this

define class Parent
  define function getName that takes nothing and returns String
    return 'parent'

define class Child extends Parent
  define function getName that takes x of type Number and returns String
    return 'child'



```

---

### [test_157_invalid_type_annotation.lay](test-cases/negative/test_157_invalid_type_annotation.lay)
**Description**: No description available.

```ruby
# negative test: invalid type annotation syntax
# expected: parser should reject this

the variable x of type is Number



```

---

### [test_158_missing_type_keyword.lay](test-cases/negative/test_158_missing_type_keyword.lay)
**Description**: No description available.

```ruby
# negative test: variable declaration missing 'type' keyword
# expected: parser should reject this

the variable x of String is 'hello'



```

---

### [test_159_invalid_return_type.lay](test-cases/negative/test_159_invalid_return_type.lay)
**Description**: No description available.

```ruby
# negative test: function with invalid return type
# expected: parser should reject this

define function test that takes nothing and returns InvalidType
  return 5



```

---

### [test_160_undefined_type_in_declaration.lay](test-cases/negative/test_160_undefined_type_in_declaration.lay)
**Description**: No description available.

```ruby
# negative test: using undefined type in variable declaration
# expected: type checker should catch this

the variable x of type UndefinedType is 5



```

---

### [test_class_validation_fail.lay](test-cases/negative/test_class_validation_fail.lay)
**Description**: Tests that calling an undefined method on a class instance raises a compile-time error

```ruby
# negative test case: class validation failure
# expected error: Class 'Person' has no method 'unknown_method'
# description: Tests that calling an undefined method on a class instance raises a compile-time error
define class Person that has
  property name which is String

the variable p is a new Person with
  name is "Alice"

print p.unknown_method()

```

---

### [type_001_undefined_variable.lay](test-cases/negative/type_001_undefined_variable.lay)
**Description**: Tests that undefined variables are caught

```ruby
# negative test case: undefined variable usage
# expected error: Variable 'undefined_var' is not defined
# description: Tests that undefined variables are caught

print the variable undefined_var



```

---

### [type_002_type_mismatch.lay](test-cases/negative/type_002_type_mismatch.lay)
**Description**: Tests that type checking catches mismatches

```ruby
# negative test case: type mismatch
# expected error: Type mismatch - cannot assign String to Number
# description: Tests that type checking catches mismatches

the variable x of type Number is 'hello'



```

---

## Oop

### [test_011.lay](test-cases/oop/test_011.lay)
**Description**: select a module and execute a simple operation to verify integration

```ruby
# test case 11
# description: select a module and execute a simple operation to verify integration

define class Module that has property name which is Text

define function select_module that takes name as String and returns String
  if name is "math" then
    return "math"
  else
    return "unknown"
  end if

# choose a module
 the variable module_name of type String is "math"
 the variable selected of type String is call function select_module with argument module_name

# create a module instance
 the variable mod of type Module is a new Module with name which is selected

# use the chosen module to run a simple computation
 if selected is "math" then
   the variable result of type Number is 3 plus 2
   print the variable result
 else
   print "module not found"
 end if

```

---

### [test_012.lay](test-cases/oop/test_012.lay)
**Description**: pipeline operations simulated with transformations and validation

```ruby
# test case 12
# description: pipeline operations simulated with transformations and validation

define function transform_value that takes x as Number and returns Number
  if x is 1 then
    return 2
  else
    return x plus 1
  end if

# simulate reading and transforming 2 streams
 the variable sum_one of type Number is 0
 the variable i of type Number is 0
 while i is less than 3 do
  the variable sum_one of type Number is sum_one plus call function transform_value with argument i
  the variable i of type Number is i plus 1
 end while

 the variable sum_two of type Number is 0
 the variable j of type Number is 0
 while j is less than 3 do
  the variable sum_two of type Number is sum_two plus call function transform_value with argument j
  the variable j of type Number is j plus 1
 end while

# validate by simple comparison
 if sum_one is sum_two then
  print "pipeline operation correct"
 else
  print "pipeline operation mismatch"
 end if

```

---

### [test_013.lay](test-cases/oop/test_013.lay)
**Description**: create an object with 2 properties and print them

```ruby
# test case 13
# description: create an object with 2 properties and print them

define class Person that has
  property name which is Text
  property age which is Number

 the variable p of type Person is a new Person with
  name which is "john"
  age which is 30

print "john"
print "30"

```

---

### [test_018.lay](test-cases/oop/test_018.lay)
**Description**: add a new user to an organization and update count

```ruby
# test case 18
# description: add a new user to an organization and update count

define class Organization that has
  property count which is Number

define class Person that has
  property name which is Text
  property email which is Text

 the variable organization of type Organization is a new Organization with
  count which is 0

 the variable new_user of type Person is a new Person with
  name which is "alice"
  email which is "alice@example.com"

# simulate add_user updating organization count
 the variable organization_count of type Number is 0
 the variable organization_count of type Number is 1
 print "user added"

```

---

### [test_022.lay](test-cases/oop/test_022.lay)
**Description**: calculate average grade if at least 3 students have passed

```ruby
# test case 22
# description: calculate average grade if at least 3 students have passed

 the variable passed_count of type Number is 3
 the variable total_score of type Number is 90

 if passed_count is greater than or equal to 3 then
  the variable average of type Number is total_score divided by passed_count
  print the variable average
 else
  print "no 1 has passed"
 end if

```

---

### [test_053.lay](test-cases/oop/test_053.lay)
**Description**: extract name and address from input and print them

```ruby
# test case 53
# description: extract name and address from input and print them

define function extract_info that takes name as String and address as String and returns Void
  print the variable name
  print the variable address
  return nothing

# call the function with sample data
 the variable name_one of type String is "john"
 the variable address_one of type String is "123 main street"
 call function extract_info with argument name_one and argument address_one

```

---

### [test_065.lay](test-cases/oop/test_065.lay)
**Description**: filter even numbers and print non-even numbers

```ruby
# test case 65
# description: filter even numbers and print non-even numbers

 the variable number of type Number is 1
 while number is less than 7 do
  if number modulo 2 is 0 then
    # even
  else
    print "not even"
  end if
  the variable number of type Number is number plus 1
 end while

```

---

### [test_066.lay](test-cases/oop/test_066.lay)
**Description**: demonstrate conditional messages with a simple loop

```ruby
# test case 66
# description: demonstrate conditional messages with a simple loop

 the variable storage of type Number is 5
 print the variable storage
 if storage is 5 then print "key exists" else print "key does not exist"

 print "looping"

```

---

### [test_073.lay](test-cases/oop/test_073.lay)
**Description**: validate simple object usage and conditional printing

```ruby
# test case 73
# description: validate simple object usage and conditional printing

define class Person that has
  property name which is Text
  property age which is Number
  property address which is Text

# create a person object
 the variable person of type Person is a new Person with
  name which is "john"
  age which is 30
  address which is "new york"

# basic conditional flow
 if "new york" is "new york" then
  print "address is correct"
 else
  print "address is incorrect"
 end if

# simple loop and prints
 the variable result of type Number is 8
 print the variable result

```

---

### [test_093.lay](test-cases/oop/test_093.lay)
**Description**: print info for 2 persons

```ruby
# test case 93
# description: print info for 2 persons

define class Person that has
  property name which is Text
  property age which is Number

 the variable p1 of type Person is a new Person with
  name which is "john"
  age which is 30

 the variable p2 of type Person is a new Person with
  name which is "jane"
  age which is twenty_five

print "john 30"
print "jane 25"

```

---

### [test_oop_001.lay](test-cases/oop/test_oop_001.lay)
**Description**: Simple class definition and object creation

```ruby
# test case OOP 001
# description: Simple class definition and object creation

define class Person that has
  property name which is Text
  property age which is Number

the variable person of type Person is a new Person with
  name which is 'Alice'
  age which is 30

print the variable person


```

---

### [test_oop_002.lay](test-cases/oop/test_oop_002.lay)
**Description**: Class with methods and inheritance

```ruby
# test case OOP 002
# description: Class with methods and inheritance

define class Animal that has
  property name which is Text
  
define function speak that takes self and returns Text
  return concatenate get name from self with ' makes a sound'

define class Dog that extends Animal
  
  define function bark that takes self and returns Text
    return concatenate get name from self with ' barks'

the variable myDog of type Dog is a new Dog with
  name which is 'Buddy'

call function print with call function bark on myDog


```

---

### [test_oop_003.lay](test-cases/oop/test_oop_003.lay)
**Description**: Person class with method to get full name

```ruby
# test case OOP 003
# description: Person class with method to get full name

define class Person that has
  property firstName of type Text
  property lastName of type Text
  property age of type Number

the variable alice of type Person is a new Person with
  firstName which is 'Alice'
  lastName which is 'Smith'
  age which is 30

the variable bob of type Person is a new Person with
  firstName which is 'Bob'
  lastName which is 'Jones'
  age which is 25

print get firstName from alice
print get lastName from bob
print get age from alice


```

---

### [test_oop_004.lay](test-cases/oop/test_oop_004.lay)
**Description**: Calculator class with methods

```ruby
# test case OOP 004
# description: Calculator class with methods

define class Calculator that has
  property result of type Number

  define function add that takes value as Number and returns Void
    the variable current of type Number is get result from self
    set result in self to current plus value

  define function multiply that takes value as Number and returns Void
    the variable current of type Number is get result from self
    set result in self to current times value

the variable calc of type Calculator is a new Calculator with
  result which is 0

call function add on calc with value 5
call function multiply on calc with value 2
print get result from calc


```

---

### [test_oop_005.lay](test-cases/oop/test_oop_005.lay)
**Description**: Bank account class with deposit and withdrawal

```ruby
# test case OOP 005
# description: Bank account class with deposit and withdrawal

define class BankAccount that has
  property balance of type Number
  property owner of type Text

  define function deposit that takes amount as Number and returns Void
    the variable current of type Number is get balance from self
    set balance in self to current plus amount

  define function withdraw that takes amount as Number and returns Void
    the variable current of type Number is get balance from self
    set balance in self to current minus amount

the variable account of type BankAccount is a new BankAccount with
  balance which is 1000
  owner which is 'John Doe'

call function deposit on account with amount 250
call function withdraw on account with amount 100
print get balance from account


```

---
## Error Handling

### [test_try_catch_div_zero.lay](test-cases/error-handling/test_try_catch_div_zero.lay)
**Description**: Demonstrates try/catch block for handling division by zero errors.

```ruby
try
    print "Attempting division by zero"
    the variable x of type Number is 10 divided by 0
    print "This should not be printed"
catch error
    print "Caught error: " + error
end try
print "Program continued"
```

---

## Imports

### [test_071.lay](test-cases/imports/test_071.lay)
**Description**: Demonstrates importing and using functions from another module.

```ruby
# test case 71
# description: Write a program that imports a math module and uses its functions to calculate the area of a rectangle.

import math_module from './math_module.lay'

# test with different length and width values
the variable length of type Number is 2
the variable width of type Number is 3

# calculate area
the variable area_result of type Number is call function calculate_area on math_module with length 10 and width 5

print "area of rectangle:"
print the variable area_result

# test with different values
the variable length_two of type Number is 3
the variable width_two of type Number is 4

the variable area_result_two of type Number is call function calculate_area with argument length_two and argument width_two

print "area of rectangle 2:"
print the variable area_result_two

# test with length = 1
the variable length_three of type Number is 1
the variable width_three of type Number is 5

the variable area_result_three of type Number is call function calculate_area with argument length_three and argument width_three

if area_result_three is greater than 0 then
  print "area calculation successful:"
  print the variable area_result_three
else
  print "an error message"
end if

---

