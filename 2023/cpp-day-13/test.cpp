#include <iostream>
#include <vector>

int main()
{
    // Example matrix initialization
    std::vector<std::vector<char>> matrix = {
        {'a', 'b', 'c'},
        {'d', 'e', 'f'},
        {'g', 'h', 'i'}};

    // Loop to print matrix elements
    for (const auto &row : matrix)
    {
        for (const auto &c : row)
        {
            std::cout << c << ' ';
        }
        std::cout << std::endl;
    }

    // Accessing matrix elements safely
    if (!matrix.empty() && !matrix[0].empty())
    {
        char a = matrix[0][0];
        std::cout << "Element at [0][0]: " << a << std::endl;
    }
    else
    {
        std::cout << "Matrix is empty or index out of range!" << std::endl;
    }

    return 0;
}
