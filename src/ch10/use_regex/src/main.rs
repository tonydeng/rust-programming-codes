// 使用exter crate regex声明引入regex包
extern crate regex;
//使用use来声明regex::Regex，要不然就要在代码中直接使用regex::Regex::new，可读性就差了许多
use regex::Regex;

const TO_SEATCH: &'static str = "
On 2017-12-31, happy. On 2018-01-01, New Year.
";

fn main() {
    // 带有捕获组的表达式
    let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
    //使用captures_iter对给定的TO)_SEATCH进行匹配和迭代，并依次捕获匹配到的字符串答应出来。
    for caps in re.captures_iter(TO_SEATCH) {
        println!("year: {}, month: {}, day {}",
        caps.get(1).unwrap().as_str(),
        caps.get(2).unwrap().as_str(),
        caps.get(3).unwrap().as_str());
    }
}


#[cfg(test)]

mod test{
    use regex::Regex;

    /**
     * 测试使用命名捕获的示例
     */
    #[test]
    fn test_named_capture() {
        
        // 在正则表达式中加上空格和注释也不影响最终匹配的结果
        // 正则表达式以(?x)为前缀，这是指定了正则表达式标记x
        // 在表达式汇总使用(?P<name>exp)这种格式来定义命名捕获组
        let re = Regex::new(r"(?x)
        (?P<year>\d{4})  # the year
        -
        (?P<month>\d{2}) # the month
        -
        (?P<day>\d{2}) # the day
        ").unwrap();
        // 使用captures方法获取匹配的捕获变量，并保存到一个HashMap中。以命名变量作为HashMap的Key，匹配的字符串作为值。
        let caps = re.captures("2019-01-01").unwrap();

        assert_eq!("2019", &caps["year"]);
        assert_eq!("01",&caps["month"]);
        assert_eq!("01",&caps["day"]);

        // 使用replace_all方法来替换匹配的字符串。注意制定的格式是以“$”符号和命名捕获变量组合而成的。
        let after= re.replace_all("2018-01-01","$month/$day/$year");
        assert_eq!(after, "01/01/2018");
    }
}