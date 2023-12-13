#include <iostream>
#include <fstream>
#include <vector>
#include <sstream>
#include <map>

int main()
{
    std::ifstream inputFile("input.ex"); // Replace "your_file.txt" with your file path

    std::vector<std::string> blocks;
    std::string block;
    std::string line;

    while (std::getline(inputFile, line))
    {
        if (line != "")
        {
            // If the line is not empty, append it to the current block
            if (!block.empty())
            {
                block += "\n"; // Add newline between lines
            }
            block += line;
        }
        else
        {
            // If the line is empty, check if the block is not empty
            if (!block.empty())
            {
                // Store the block in the vector and reset block variable
                blocks.push_back(block);
                block = "";
            }
        }
    }

    // If there's a block left after reading the file, add it to the vector
    if (!block.empty())
    {
        blocks.push_back(block);
    }

    int blockNum = 1;
    std::map<int, std::vector<int>> lineIndexes;
    std::map<int, std::vector<int>> columnIndexes;

    for (const std::string &block : blocks)
    {
        int lineNumber = 0;

        // std::cout << "Block " << blockNum++ << ":\n";

        std::istringstream blockStream(block);
        std::string line, prevLine;
        std::vector<int> duplicates;

        // Read lines from the block stream
        while (std::getline(blockStream, line))
        {
            if (line == prevLine)
            {
                std::cout << "Duplicate line: \"" << line << " " << lineNumber << "\" found.\n";
                duplicates.push_back(lineNumber);
            }
            prevLine = line;
            lineNumber++;
        }
        if (!duplicates
                 .empty())
        {

            lineIndexes.insert(std::pair<int, std::vector<int>>(blockNum, duplicates));
        }
        // Vector to store lines of characters
        std::vector<std::vector<char>> matrix;

        // Splitting the string into lines based on newline characters

        // std::string line;
        std::string clonedBlock(block);

        size_t pos = clonedBlock.find("\n");

        while ((pos) != std::string::npos)
        {
        std:
            std::string newline = clonedBlock.substr(0, pos);
            // std::cout << line << std::endl;
            std::vector<char> row; // Vector to hold characters of a line
            // Iterate through each character in the line and push into the row vector
            for (char c : newline)
            {
                row.push_back(c);
            }

            matrix.push_back(row); // Push the row vector into the matrix

            if (pos + 1 < clonedBlock.size())
            {
                clonedBlock = clonedBlock.substr(pos + 1); // Update 'block' with content after '\n' for the next line
            }
            else
            {
                break; // Break the loop if no more '\n' found to avoid out-of-range error
            }
            pos = clonedBlock.find("\n");
        }

        // std::cout << "Printing matrix: " << std::endl;

        // for (const auto &elem : matrix)
        // {
        //     for (const auto &c : elem)
        //     {
        //         std::cout << c;
        //     }
        //     std::cout << std::endl;
        // }
        // std::cout << "End of printing matrix " << std::endl;

        for (size_t j = 0; j < matrix[0].size() - 1; ++j)
        {
            std::vector<int> vec;
            bool equal = true;

            for (size_t i = 0; i < matrix.size(); ++i)
            {
                if (matrix[i][j] != matrix[i][j + 1])
                {
                    equal = false;
                }
            }
            if (equal == true)
            {
                vec.push_back(j);
                std::cout << "Duplicate Column: \"" << j
                          << " found.\n";
            }
            if (!vec.empty())
            {
                columnIndexes.insert(std::pair<int, std::vector<int>>(blockNum, vec));
            }
        }

        blockNum++;
    }

    int sum = 0;
    for (const auto &pair : columnIndexes)
    {
        for (int index : pair.second)
        {
            sum += index;
        }
    }

    for (const auto &pair : lineIndexes)

    {
        for (int index : pair.second)
        {

            sum += index * 100;
        }
        std::cout << "Sum: " << sum << std::endl;
    }

    std::cout << "Total: " << sum;
    std::cout
        << std::endl;
    return 0;
}