import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;

public class Main 
{ 
    private static int fuel_need(int mass) {
        return (mass / 3) - 2;
    }

    private static int fuel_need2(int mass) {
        int t = 0;
        while (mass > 0) {
            int fuel = fuel_need(mass);
            if(fuel <= 0) {
                break;
            }
            t += fuel;
            mass = fuel;
        }

        return t;
    }
    private static void run_tests() {
        assert fuel_need(12) == 2;
        assert fuel_need(14) == 2;
        assert fuel_need(1969) == 654;
        assert fuel_need(100756) == 33583;

        assert fuel_need2(12) == 2;
        assert fuel_need2(14) == 2;
        assert fuel_need2(1969) == 966;
        assert fuel_need2(100756) == 50346;
        System.out.println("Tests all good!");
    }

    public static void main(String [] args)
    {
        if(args.length < 1) {
            run_tests();
            return;
        }

        BufferedReader reader;
        int total = 0;
        try {
            System.out.println("Reading " + args[0]);
            reader = new BufferedReader(new FileReader(args[0]));
            String line;
            while (true) {
                line = reader.readLine();
                if (line == null) {
                    break;
                }
                total += fuel_need(Integer.parseInt(line));
            }
            reader.close();
        } catch (IOException e) {
            e.printStackTrace();
        }

        System.out.println("Part#1 Total: " + total);

        total = 0;
        try {
            System.out.println("Reading " + args[0]);
            reader = new BufferedReader(new FileReader(args[0]));
            String line;
            while (true) {
                line = reader.readLine();
                if (line == null) {
                    break;
                }
                total += fuel_need2(Integer.parseInt(line));
            }
            reader.close();
        } catch (IOException e) {
            e.printStackTrace();
        }

        System.out.println("Part#2 Total: " + total);
    }

}

