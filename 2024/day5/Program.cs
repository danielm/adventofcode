class Program
{
    static void Main(string[] args)
    {
        string filePath = args[0];

        Console.WriteLine("Advent of Code 2024 - Day 5");

        var rules = new List<int[]>();
        var updates = new List<int[]>();
        
        bool isNumbersSection = false;
        
        // Read file structure
        foreach (var line in File.ReadLines(filePath))
        {
            if (string.IsNullOrWhiteSpace(line))
            {
                isNumbersSection = true;
                continue;
            }
            
            if (!isNumbersSection)
            {
                var parts = line.Split('|');
                if (parts.Length == 2 && int.TryParse(parts[0], out int part1) && int.TryParse(parts[1], out int part2))
                {
                    rules.Add(new int[] { part1, part2 });
                }
            }
            else
            {
                var parts = line.Split(',');
                var numberList = new List<int>();
                foreach (var part in parts)
                {
                    if (int.TryParse(part.Trim(), out int number))
                    {
                        numberList.Add(number);
                    }
                }
                updates.Add(numberList.ToArray());
            }
        }
        
        int partIAnswer = 0;
        int partIIAnswer = 0;

        // Cada Update
        foreach (var update in updates)
        {
            if (ValidUpdate(update, rules)) {
                partIAnswer += update[update.Length / 2];
            } else {
                // Part II --> Reorder

                partIIAnswer += ReorderUpdateRecursive(update, rules)[update.Length / 2];
                
            }
        }

        Console.WriteLine($"First part is: {partIAnswer}");
        Console.WriteLine($"Second part is: {partIIAnswer}");
    }

    static bool ValidUpdate(int[] update, List<int[]> rules)
    {
        foreach (var rule in rules)
        {
            var left  = Array.IndexOf(update, rule[0]);
            var right  = Array.IndexOf(update, rule[1]);

            // if any of the elements isn't in the current checking rule, just pass since it doesnt apply
            if (left < 0 || right < 0) {
                continue;
            }

            // We are comparing the indexes of each elemnts  not the values themselves
            // if the left elemet is a in higer index then the right element, it is not valid
            if (left > right) {
                return false;
            }
        }

        return true;
    }


    static int[] ReorderUpdateRecursive(int[] update, List<int[]> rules, int idx = 0)
    {
        // Last rule just was checked, we got our reordered update
        if (idx >= rules.Count)
        {
            return update;
        }

        var rule = rules[idx];
        var left = Array.IndexOf(update, rule[0]);
        var right = Array.IndexOf(update, rule[1]);

        // the >=0 is that if the elment in the rule exist in our update
        // the left > reight checks if the order is wrong, basically the oposite of Part I
        if (left >= 0 && right >= 0 && left > right)
        {
            // Swap elements
            var tmp = update[left];
            update[left] = update[right];
            update[right] = tmp;

            // Restart from the first rule after swap
            return ReorderUpdateRecursive(update, rules, 0);
        }

        // Move to the next rule
        return ReorderUpdateRecursive(update, rules, idx + 1);
    }
}
