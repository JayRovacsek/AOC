using System;
using System.Collections.Generic;
using System.IO;

namespace advent_csharp
{
    class Program
    {
        static void Main(string[] args)
        {
            var programLogic = new ProgramLogic();
            var claims = programLogic.LoadClaims("input.txt");
            var matrix = programLogic.GenerateMatrix(claims);
            var fabricSquares = programLogic.CountDoubleClaims(matrix);

            Console.WriteLine($"Fabric tiles count used by two or more claims is: {fabricSquares}");

            var claimNotOverlapping = programLogic.DetermineNonOverlapping(claims);

            Console.WriteLine($"Fabric claim not overlapping with any other claim is: {claimNotOverlapping}");
            Console.WriteLine("Press any key to end");
            Console.ReadKey();
        }
    }
}
