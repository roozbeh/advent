import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;
import java.util.HashMap;
import java.util.Hashtable;
import java.util.stream.Stream;

import static java.lang.Math.abs;

class Index {

    private int x;
    private int y;

    public Index(int x, int y) {
        this.x = x;
        this.y = y;
    }

    @Override
    public int hashCode() {
        return this.x ^ this.y;
    }

    @Override
    public boolean equals(Object obj) {
        if (this == obj)
            return true;
        if (obj == null)
            return false;
        if (getClass() != obj.getClass())
            return false;
        Index other = (Index) obj;
        if (x != other.x)
            return false;
        if (y != other.y)
            return false;
        return true;
    }
}

public class Advent03
{

    private static void run_tests() {
        // assert fuel_need(12) == 2;
        System.out.println("Tests all good!");
    }

    public static void main(String [] args)
    {
        if(args.length < 1) {
            run_tests();
            return;
        }

        BufferedReader reader;

        try {
            System.out.println("Code " + args[0]);
            reader = new BufferedReader(new FileReader(args[0]));
            String line1 = reader.readLine();
            String line2 = reader.readLine();
            if (line1 == null || line2 == null) {
                return;
            }
            reader.close();

            String[] split = line1.split(",");
            int x=0,y=0;
            HashMap<Index, Integer> wire1 = new HashMap<Index, Integer>();
            for (String s : split) {
                char direction = s.charAt(0);
                int count = Integer.parseInt(s.substring(1));
                System.out.printf("direction: %c , count: %d\n", direction, count);

                for (int i=0;i<count;i++) {
                    if (direction == 'R') {
                        x += 1;
                    }
                    if (direction == 'L') {
                        x -= 1;
                    }
                    if (direction == 'U') {
                        y -= 1;
                    }
                    if (direction == 'D') {
                        y += 1;
                    }

                    Index index = new Index(x, y);
                    wire1.put(index, 1);
                }
            }


            String[] split2 = line2.split(",");
            int min_distance = 0;
            x=0;y=0;
            for (String s : split2) {
                char direction = s.charAt(0);
                int count = Integer.parseInt(s.substring(1));
                System.out.printf("direction: %c , count: %d\n", direction, count);

                for (int i=0;i<count;i++) {
                    if (direction == 'R') {
                        x += 1;
                    }
                    if (direction == 'L') {
                        x -= 1;
                    }
                    if (direction == 'U') {
                        y -= 1;
                    }
                    if (direction == 'D') {
                        y += 1;
                    }

                    Index index = new Index(x, y);
                    if (wire1.containsKey(index)) {
                        System.out.printf("Collision %d, %d\n", x, y);
                        int d = abs(x) + abs(y);
                        if (min_distance == 0 || d < min_distance) {
                            min_distance = d;
                        }
                    }
                }
            }
            System.out.println("min distance: " + min_distance);




//
//            int x = 0, y = 0;
//
//            Integer []instructions = Stream.of(orig_instructions).toArray(Integer[]::new);
//            instructions[1] = 12;
//            instructions[2] = 2;
//            calcInput(instructions);
//            System.out.println("Part#1 output: " + instructions[0]);


        } catch (IOException e) {
            e.printStackTrace();
        }


    }

}