class Solution(object):
    def numberOfEmployeesWhoMetTarget(self, hours, target):

        employees = 0
        for h in hours:
            if h >= target:
                employees = employees + 1
        return employees
