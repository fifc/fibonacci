using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace ConsoleApp1
{
    class Program
    {
        static int fibonacci(int n)
        {
            if (n < 2)
                return n;
            return fibonacci(n - 2) + fibonacci(n - 1);
        }
        static void Main(string[] args)
        {
            var start = DateTime.Now;
            Console.WriteLine("fibn: {0}, time: {1}", fibonacci(42), (DateTime.Now - start).TotalMilliseconds);
            Console.ReadKey();
        }
    }
}

