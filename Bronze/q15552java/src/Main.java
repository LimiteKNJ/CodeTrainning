import java.io.*;

public class Main {
    public static void main(String[] args) {
        BufferedWriter output = new BufferedWriter(new OutputStreamWriter(System.out));
        BufferedReader input = new BufferedReader(new InputStreamReader(System.in));
        StringBuilder result = new StringBuilder();

        try {
            int caseCnt = Integer.parseInt(input.readLine().trim());
            for (int i = 0; i < caseCnt; i++){
                String[] nums = input.readLine().split(" ");
                int a = Integer.parseInt(nums[0]);
                int b = Integer.parseInt(nums[1]);

                result.append((a + b) + "\n");
            } output.write(result.toString()); output.flush();
            input.close(); output.close();
        } catch (Exception e) {
            e.printStackTrace();
        }
    }
}