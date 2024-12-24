import json

def process_test_cases(file_path):
    # List to store test case objects
    test_cases = []
    
    # Open the file for reading
    with open(file_path, 'r') as file:
        # Initialize temporary variables to store values
        dsCAVS = deIUT = Z = None
        
        for line in file:
            line = line.strip()
            
            # Skip empty lines or lines that contain brackets
            if not line or '[' in line:
                continue

            # Check if the line contains dsCAVS, deIUT, or Z and extract the values
            if line.startswith("dsCAVS"):
                dsCAVS = line.split('=')[1].strip()
            elif line.startswith("deIUT"):
                deIUT = line.split('=')[1].strip()
            elif line.startswith("Z"):
                Z = line.split('=')[1].strip()

            # Once we have all three values, create an object and add it to the list
            if dsCAVS and deIUT and Z:
                test_case = {
                    "private_key1": dsCAVS,
                    "private_key2": deIUT,
                    "shared_secret": Z
                }
                test_cases.append(test_case)
                
                # Reset for the next test case
                dsCAVS = deIUT = Z = None

    return test_cases

def generate_rust_code(test_vectors):
    rust_code = "const TEST_VECTORS: [TestVector; {}] = [\n".format(len(test_vectors))
    for vec in test_vectors:
        rust_code += "    TestVector {\n"
        rust_code += f'        private_key1: "{vec["private_key1"]}",\n'
        rust_code += f'        private_key2: "{vec["private_key2"]}",\n'
        rust_code += f'        shared_secret: "{vec["shared_secret"]}",\n'
        rust_code += "    },\n"
    rust_code += "];\n"
    return rust_code

# Example usage
file_path = 'p521.txt'  # Replace with the path to your text file
json_string = process_test_cases(file_path)
print(generate_rust_code(json_string))
