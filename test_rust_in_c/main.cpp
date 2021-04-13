#include <string>
#include <vector>
#include <cinttypes>
#include <fstream>

extern "C" void validator_wasm_with_path(char *s);
extern "C" void validator_wasm_with_content(uint8_t *s, int32_t size);

// test add_one
extern "C" void test_wasm_file_path(char *s);
extern "C" void test_wasm_file_content(uint8_t *s, int32_t bytes_size);

void get_wasm_bytes(const char *file_path, bool test = false)
{
    uint8_t *bytes;
    uint32_t bytes_size;

    std::ifstream file_size(file_path, std::ifstream::ate | std::ifstream::binary);
    bytes_size = file_size.tellg();

    file_size.close();

    std::ifstream in(file_path, std::ifstream::binary);
    bytes = (uint8_t *)malloc(bytes_size);
    in.read(reinterpret_cast<char *>(bytes), bytes_size);
    in.close();
    // std::string str;
    // std::vector<uint8_t> str;
    printf("file size: %d\n", bytes_size);
    // for (auto index = 0; index < bytes_size; ++index)
    // {
    // printf("%u ", *(bytes + index));
    // str.push_back(static_cast<uint8_t>(*(bytes + index)));
    // }
    // printf("\n");
    if (test)
    {
        test_wasm_file_content(bytes, bytes_size);
    }
    else
    {
        validator_wasm_with_content(bytes, bytes_size);
    }
}

int main()
{
    // test_str("test_str");
    // get_wasm_bytes("./example/add.wasm");


    get_wasm_bytes("./example/wadd.wasm", true);
    test_wasm_file_path("./example/wadd.wasm");

    get_wasm_bytes("./example/wadd.wasm");
    validator_wasm_with_path("./example/wadd.wasm");

    // validator_wasm_with_path("./example/add1.wasm");
    // validator_wasm_with_path("./example/fib.wasm");
    // validator_wasm_with_path("./example/add.wasm");
    return 0;
}