using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;

namespace advent_csharp
{
    public class ProgramLogic
    {
        public IEnumerable<Claim> LoadClaims(string filename)
        {
            var claims = new List<Claim>();
            var line = string.Empty;

            using (var sr = new StreamReader(filename))
            {
                while ((line = sr.ReadLine()) != null)
                {
                    var parts = line.Split(' ');
                    claims.Add(new Claim
                    {
                        Id = Convert.ToInt32(parts[0].Remove(0, 1)),
                        X = Convert.ToInt32(parts[2].Split(',')[0]),
                        Y = Convert.ToInt32(parts[2].Split(',')[1].TrimEnd(':')),
                        Width = Convert.ToInt32(parts[3].Split('x')[0]),
                        Height = Convert.ToInt32(parts[3].Split('x')[1])
                    });
                }
            }
            return claims;
        }

        public int GetMaxX(IEnumerable<Claim> claims)
        {
            return claims.Max(x => x.X + x.Width);
        }

        public int GetMaxY(IEnumerable<Claim> claims)
        {
            return claims.Max(x => x.Y + x.Height);
        }

        public int[,] GenerateMatrix(IEnumerable<Claim> claims)
        {
            var matrix = new int[GetMaxY(claims)+1, GetMaxY(claims)+1];
            foreach(var claim in claims)
                for(int i = claim.X; i < claim.X+claim.Width; i++)
                    for (int j = claim.Y; j < claim.Y + claim.Height; j++)
                        matrix[i,j]++;
            return matrix;
        }

        public int CountDoubleClaims(int[,] matrix)
        {
            var count = 0;
            for(int i = 0; i < matrix.GetLength(0);i++)
                for (int j = 0; j < matrix.GetLength(1); j++)
                    if(matrix[i,j] >= 2)
                        count++;
            return count;
        }

        public int DetermineNonOverlapping(IEnumerable<Claim> claims)
        {
            var c = claims.ToList();

            foreach(var claim in claims)
                for(int i=0; i < c.Count(); i++)
                    if (Intersects(claim, c[i]) && claim.Id != c[i].Id)
                    {
                        claim.IntersectAnotherClaim = true;
                        break;
                    }

            return claims.Select(x => x).Where(x => x.IntersectAnotherClaim == false).Select(x => x.Id).FirstOrDefault();
        }

        private bool Intersects(Claim claim, Claim challenger)
        {
            for(int i = claim.X;i<= claim.X + claim.Width; i++)
                for (int j = claim.Y; j <= claim.Y + claim.Height; j++)
                    if (i > challenger.X && i < challenger.X + challenger.Width)
                        if (j > challenger.Y && j < challenger.Y + challenger.Height)
                            return true;
            return false;
        }
    }
}
