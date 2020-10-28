using System;
using System.Collections.Generic;
using System.Text;

namespace advent_csharp
{
    public class Claim
    {
        public int Id { get; set; }
        public int X { get; set; }
        public int Y { get; set; }
        public int Width { get; set; }
        public int Height { get; set; }
        public bool IntersectAnotherClaim { get; set; } = false;
    }
}
