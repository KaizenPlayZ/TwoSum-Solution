package org.example;
public class Main {
  public static void main(String[] args) {
    System.out.println(Main.twoSum(new int[] {1,2},3));
  }
  public static int[] twoSum(int[] nums,int target) throws IllegalArgumentException {
        for (int i = 0; i < nums.length; i++) {
            for (int j = i + 1; j < nums.length; j++) {
                if (nums[j] == target - nums[i]) {
                    return new int[] {i,j};
                }
            }
        }
        return new int[] {0,0};
    }
}
