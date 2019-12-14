import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;
import java.util.stream.Stream;

public class Main 
{ 
    private static void run_tests() {
        // assert fuel_need(12) == 2;
        System.out.println("Tests all good!");
    }

    private static void calcInput(Integer[] instructions) {
        int cursor = 0;
        while(true) {
            int inst = instructions[cursor];
            if (inst == 99) {
//                System.out.println("end");
                return;
            }

            if (inst == 1) {
//                System.out.printf("[%d] + [%d]\n", instructions[cursor+1], instructions[cursor+2]);
                instructions[instructions[cursor+3]] =
                        instructions[instructions[cursor+1]] + instructions[instructions[cursor+2]];
            }

            if (inst == 2) {
//                System.out.printf("[%d] * [%d]\n", instructions[cursor+1], instructions[cursor+2]);
                instructions[instructions[cursor+3]] =
                        instructions[instructions[cursor+1]] * instructions[instructions[cursor+2]];
            }

            cursor += 4;
        }
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
            String line = reader.readLine();
            if (line == null) {
                return;
            }
            reader.close();

            String[] split = line.split(",");
            Integer []orig_instructions = Stream.of(split).map(Integer::valueOf).toArray(Integer[]::new);


            Integer []instructions = Stream.of(orig_instructions).toArray(Integer[]::new);
            instructions[1] = 12;
            instructions[2] = 2;
            calcInput(instructions);
            System.out.println("Part#1 output: " + instructions[0]);

            for (int i=0;i<99; i++) {
                for (int j=0;j<99; j++) {
                    instructions = Stream.of(orig_instructions).toArray(Integer[]::new);
                    instructions[1] = i;
                    instructions[2] = j;
                    calcInput(instructions);
                    if (instructions[0] == 19690720) {
                        System.out.println("noun: " + i + " , verb: " + j + " => " + (100*i+j));
                        return;
                    }
                }

            }

        } catch (IOException e) {
            e.printStackTrace();
        }


     }

}